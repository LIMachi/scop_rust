use std::collections::HashMap;
use std::os::raw::c_void;
use gl::types::GLint;
use crate::maths::matrix::{Mat4, Matrix};
use crate::maths::transform::Transform;
use crate::opengl::enums::Shaders;
use crate::opengl::safe_calls;
use crate::opengl::shader::{ShaderProgram, ShaderProgramBuilder};
use crate::opengl::uniform::Uniform;
use crate::other::resource_manager::ResourceManager;

#[derive(Debug, Copy, Clone)]
pub struct ObjectId {
    model: usize,
    batch: usize,
    index: usize
}

#[derive(Debug, Copy, Clone, Default)]
pub struct ObjectData {
    pub transform: Transform,
    pub flags: i32,
}

#[derive(Debug)]
pub enum ObjectChange {
    None,
    Transform(Transform),
    TransformFlags(Transform, i32),
    TransformFlagsVisibility(Transform, i32, bool),
    Flags(i32),
    FlagsVisibility(i32, bool),
    Visibility(bool),
    TransformVisibility(Transform, bool),
    Remove
}

#[derive(Debug)]
struct PickingHandler {
    shader: ShaderProgram,
    camera_uniform: Uniform,
    projection_uniform: Uniform,
    instances_uniform: Uniform,
    id_uniform: Uniform
}

impl PickingHandler {
    fn new() -> Self {
        let shader = ShaderProgramBuilder::default()
            .add_shader(Shaders::Vertex, include_str!("picking.vert"))
            .add_shader(Shaders::Fragment, include_str!("picking.frag"))
            .build().unwrap();
        Self {
            camera_uniform: shader.uniform("camera"),
            projection_uniform: shader.uniform("projection"),
            instances_uniform: shader.uniform("object"),
            id_uniform: shader.uniform("id"),
            shader,
        }
    }
}

pub const MAX_BATCH_SIZE: usize = 128;

#[derive(Debug)]
struct Batch { //missing system that merge 2 batches if they have both their len < MAX_BATCH_SIZE / 2
    len: usize, //total amount of entities in this batch (including invisible ones that are not sent to gpu)
    visible: usize, //total visible entities (real amount of entities to be drawn by the gpu, might be lower than len)
    data: [ObjectData; MAX_BATCH_SIZE],
    raw_mat: [f32; MAX_BATCH_SIZE * 16],
    raw_flags: [i32; MAX_BATCH_SIZE]
}

#[derive(Debug)]
pub struct Scene {
    camera: Transform,
    camera_uniform: Uniform,
    projection_uniform: Uniform,
    instances: HashMap<usize, Vec<Batch>>,
    instances_uniform: Uniform,
    flags_uniform: Uniform,
    shader: ShaderProgram,
    picking_handler: PickingHandler,
    // light_debug_shader: ShaderProgram
}

impl Scene {
    pub fn new(shader: ShaderProgram) -> Self {
        let picking_handler = PickingHandler::new();
        shader.set_active();
        Self {
            camera: Transform::default(),
            camera_uniform: shader.uniform("camera"),
            projection_uniform: shader.uniform("projection"),
            instances: HashMap::new(),
            instances_uniform: shader.uniform("object"),
            flags_uniform: shader.uniform("flags"),
            shader,
            picking_handler,
            // light_debug_shader: ShaderProgramBuilder::default().add_shader(Shaders::Vertex, include_str!("picking.vert")).add_shader(Shaders::Fragment, include_str!("picking.frag")).build().unwrap(),
        }
    }
    
    pub fn set_projection(&mut self, fov: f32, aspect_ratio: f32) {
        let proj = Matrix::projection(fov.to_radians(), aspect_ratio, 0.01, 1000.);
        self.projection_uniform.mat4(proj);
        self.picking_handler.shader.set_active();
        self.picking_handler.projection_uniform.mat4(proj);
        self.shader.set_active();
    }
    
    pub fn set_camera(&mut self, camera: Transform) {
        let mat = camera.as_view_matrix();
        self.camera_uniform.mat4(mat);
        self.picking_handler.shader.set_active();
        self.picking_handler.camera_uniform.mat4(mat);
        self.shader.set_active();
        self.camera = camera;
    }
    
    pub fn get_camera(&self) -> Transform { self.camera }
    
    pub fn spawn_object(&mut self, model: usize, transform: Transform, flags: i32) -> ObjectId {
        let v = self.instances.entry(model).or_default();
        for (batch, Batch { len, visible, data, raw_mat, raw_flags }) in v.iter_mut().enumerate() {
            if *len < MAX_BATCH_SIZE {
                data[*len] = ObjectData {
                    transform,
                    flags,
                };
                Mat4::from(transform).raw_copy(&mut raw_mat[16 * *len .. 16 * *len + 16]);
                raw_flags[*len] = flags;
                *len += 1;
                *visible += 1;
                return ObjectId {
                    model,
                    batch,
                    index: *len - 1,
                };
            }
        }
        let mut data = [ObjectData::default(); MAX_BATCH_SIZE];
        data[0] = ObjectData {
            transform,
            flags,
        };
        let mut raw_mat = [0f32; 16 * MAX_BATCH_SIZE];
        Mat4::from(transform).raw_copy(&mut raw_mat[0 .. 16]);
        let mut raw_flags = [0i32; MAX_BATCH_SIZE];
        raw_flags[0] = flags;
        v.push(Batch {
            len: 1,
            visible: 1,
            data,
            raw_mat,
            raw_flags,
        });
        ObjectId {
            model,
            batch: v.len() - 1,
            index: 0
        }
    }
    
    pub fn pick(&mut self, resources: &ResourceManager, pixel_x: usize, pixel_y: usize) -> Option<ObjectId> {
        let mut acc_vec = Vec::new();
        safe_calls::clear_screen();
        self.picking_handler.shader.set_active();
        for (model, batches) in self.instances.iter_mut() {
            if let Some(mpm) = resources.get_multipart_model(*model) {
                for (batch, Batch { len, visible, data, raw_mat, raw_flags }) in batches.iter_mut().enumerate() {
                    self.picking_handler.instances_uniform.raw_array_mat4(&raw_mat[0..*len * 16]);
                    self.picking_handler.id_uniform.int(acc_vec.len() as i32);
                    mpm.draw_instances(*len);
                    for index in 0..*len {
                        acc_vec.push(ObjectId {
                            model: *model,
                            batch,
                            index,
                        });
                    }
                }
            }
        }
        let t = [0u8; 4];
        unsafe {
            gl::Flush();
            gl::Finish();
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            gl::ReadPixels(pixel_x as GLint, pixel_y as GLint, 1, 1, gl::RGBA, gl::UNSIGNED_BYTE, t.as_ptr() as *mut f32 as *mut c_void);
        }
        let id = t[3] as usize | ((t[2] as usize) << 8) | ((t[1] as usize) << 16) | ((t[0] as usize) << 24);
        self.shader.set_active();
        if id > 0 && id <= acc_vec.len() {
            Some(acc_vec[id - 1])
        } else {
            None
        }
    }
    
    pub fn draw(&mut self, resources: &ResourceManager) {
        for (model, batches) in self.instances.iter_mut() {
            if let Some(mpm) = resources.get_multipart_model(*model) {
                for Batch { len, visible, data, raw_mat, raw_flags } in batches {
                    self.instances_uniform.raw_array_mat4(&raw_mat[0..*visible * 16]);
                    self.flags_uniform.array_int(&raw_flags[0..*visible]);
                    mpm.draw_instances(*visible);
                }
            }
        }
    }
    
    //return true: object changed place (visibility change) or was removed
    //len of batch should be queried again, and iteration should go over this id again
    fn do_run<F: FnMut(ObjectId, ObjectData) -> ObjectChange>(id: ObjectId, batch: &mut Batch, runner: &mut F) -> u8 {
        match runner(id, batch.data[id.index]) {
            ObjectChange::None => 0,
            ObjectChange::Transform(tr) => {
                batch.data[id.index].transform = tr;
                Mat4::from(tr).raw_copy(&mut batch.raw_mat[id.index * 16 .. id.index * 16 + 16]);
                0
            }
            ObjectChange::TransformFlags(tr, f) => {
                batch.data[id.index].transform = tr;
                Mat4::from(tr).raw_copy(&mut batch.raw_mat[id.index * 16 .. id.index * 16 + 16]);
                batch.data[id.index].flags = f;
                batch.raw_flags[id.index] = f;
                0
            }
            ObjectChange::TransformFlagsVisibility(tr, f, v) => {
                batch.data[id.index].transform = tr;
                Mat4::from(tr).raw_copy(&mut batch.raw_mat[id.index * 16 .. id.index * 16 + 16]);
                batch.data[id.index].flags = f;
                batch.raw_flags[id.index] = f;
                1
            }
            ObjectChange::Flags(f) => {
                batch.data[id.index].flags = f;
                batch.raw_flags[id.index] = f;
                0
            }
            ObjectChange::FlagsVisibility(f, v) => {
                batch.data[id.index].flags = f;
                batch.raw_flags[id.index] = f;
                1
            }
            ObjectChange::Visibility(v) => 1,
            ObjectChange::TransformVisibility(tr, v) => {
                batch.data[id.index].transform = tr;
                Mat4::from(tr).raw_copy(&mut batch.raw_mat[id.index * 16 .. id.index * 16 + 16]);
                1
            }
            ObjectChange::Remove => {
                2
            }
        }
    }
    
    //seems bugged for now, need to rethink how i did this
    //intelligently reorganize data to remove multiple object from the batch at once
    fn batch_remove(batch: &mut Batch, remove: [bool; MAX_BATCH_SIZE]) {
        let mut offset = 0;
        for i in 0..batch.len {
            if offset > 0 && i >= offset {
                batch.data[i - offset] = batch.data[i];
                batch.raw_flags[i - offset] = batch.raw_flags[i];
                for t in 0..16 {
                    batch.raw_mat[(i - offset) * 16 + t] = batch.raw_mat[i * 16 + t];
                }
            }
            if remove[i] {
                offset += 1;
                if i < batch.visible {
                    batch.visible -= 1;
                }
            }
        }
        batch.len -= offset;
    }
    
    pub fn run_on_instance<F: FnMut(ObjectId, ObjectData) -> ObjectChange>(&mut self, id: ObjectId, mut runner: F) {
        if let Some(batches) = self.instances.get_mut(&id.model) {
            if let Some(batch) = batches.get_mut(id.batch) {
                if id.index < batch.len {
                    let mut remove = [false; MAX_BATCH_SIZE];
                    if Self::do_run(id, batch, &mut runner) == 2 {
                        remove[id.index] = true;
                        Self::batch_remove(batch, remove);
                    }
                }
            }
        }
    }

    pub fn run_on_instances<F: FnMut(ObjectId, ObjectData) -> ObjectChange>(&mut self, mut runner: F) {
        for (model, batches) in &mut self.instances {
            for (batch_id, batch) in batches.iter_mut().enumerate() {
                let mut index = 0;
                let mut remove = [false; MAX_BATCH_SIZE];
                let mut will_remove = false;
                while index < batch.len {
                    let r = Self::do_run(ObjectId {
                        model: *model,
                        batch: batch_id,
                        index,
                    }, batch, &mut runner);
                    if r == 2 {
                        will_remove = true;
                        remove[index] = true;
                    }
                    if r != 1 {
                        index += 1;
                    }
                }
                if will_remove {
                    Self::batch_remove(batch, remove);
                }
            }
        }
    }
}