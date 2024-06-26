use std::collections::HashMap;
use std::os::raw::c_void;
use gl::types::GLint;
use crate::maths::matrix::{Mat4, Matrix};
use crate::maths::transform::Transform;
use crate::opengl::enums::Shaders;
use crate::opengl::objectv2::MultiPartModel;
use crate::opengl::safe_calls;
use crate::opengl::shader::{ShaderProgram, ShaderProgramBuilder};
use crate::opengl::uniform::Uniform;
use crate::other::handles::{Get, GetMut, Handle, Set};

#[derive(Debug)]
pub struct ObjectData {
    // scene: Handle<Scene>,
    model: Handle<MultiPartModel>,
    transform: Transform,
    flags: i32,
}

impl Get<Transform> for ObjectData {
    fn get(&self) -> &Transform { &self.transform }
}

impl GetMut<Transform> for ObjectData {
    fn get_mut(&mut self) -> &mut Transform { &mut self.transform }
}

impl Get<i32> for ObjectData {
    fn get(&self) -> &i32 { &self.flags }
}

impl GetMut<i32> for ObjectData {
    fn get_mut(&mut self) -> &mut i32 { &mut self.flags }
}

impl Set<Transform> for ObjectData {
    fn set(&mut self, value: Transform) { self.transform = value; }
}

impl Set<i32> for ObjectData {
    fn set(&mut self, value: i32) { self.flags = value; }
}

impl Get<Handle<MultiPartModel>> for ObjectData {
    fn get(&self) -> &Handle<MultiPartModel> { &self.model }
}

impl GetMut<Handle<MultiPartModel>> for ObjectData {
    fn get_mut(&mut self) -> &mut Handle<MultiPartModel> { &mut self.model }
}

impl Clone for ObjectData {
    fn clone(&self) -> Self {
        Self {
            // scene: self.scene.clone_weak(),
            model: self.model.clone_weak(),
            transform: self.transform,
            flags: self.flags
        }
    }
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

#[derive(Debug)]
pub struct Scene {
    camera: Transform,
    camera_uniform: Uniform,
    projection_uniform: Uniform,
    instances: HashMap<Handle<MultiPartModel>, (Vec<Handle<ObjectData>>, Vec<f32>, Vec<i32>)>,
    instances_uniform: Uniform,
    flags_uniform: Uniform,
    shader: ShaderProgram,
    picking_handler: PickingHandler,
    // light_debug_shader: ShaderProgram
}

impl Scene {
    pub const MAX_BATCH_SIZE: usize = 128;
    
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
    
    pub fn spawn_object(&mut self, model: &Handle<MultiPartModel>, transform: Transform, flags: i32) -> Handle<ObjectData> {
        let (instances, raw_mat, f) = self.instances.entry(model.clone_weak()).or_default();
        let instance = Handle::new_strong(ObjectData {
            model: model.clone_weak(),
            transform,
            flags
        });
        let out = instance.clone_weak();
        instances.push(instance);
        raw_mat.extend(Vec::<f32>::from(Mat4::from(transform)));
        f.push(flags);
        out
    }

    pub fn despawn_object(&mut self, mut object: Handle<ObjectData>) {
        if object.present() {
            object.get_mut::<Handle<MultiPartModel>>().manual_dirty();
            if let Some(i) = self.instances.get_mut(&object.get()) {
                i.0.retain(|v| v != &object);
            }
        }
    }
    
    pub fn pick(&mut self, pixel_x: usize, pixel_y: usize) -> Handle<ObjectData> {
        let mut acc_vec = Vec::new();
        safe_calls::clear_screen();
        self.picking_handler.shader.set_active();
        for (o, i) in self.instances.iter_mut() {
            if o.present() {
                Self::pre_draw(o, i);
                if i.0.len() < Self::MAX_BATCH_SIZE {
                    self.picking_handler.instances_uniform.raw_array_mat4(&i.1);
                    self.picking_handler.id_uniform.int(acc_vec.len() as i32);
                    o.draw_instances(i.0.len());
                } else {
                    for batch in 0 .. i.0.len() / Self::MAX_BATCH_SIZE {
                        self.picking_handler.instances_uniform.raw_array_mat4(&i.1[batch * Self::MAX_BATCH_SIZE * 16 .. (batch * Self::MAX_BATCH_SIZE + Self::MAX_BATCH_SIZE) * 16]);
                        self.picking_handler.id_uniform.int(acc_vec.len() as i32 + (batch * Self::MAX_BATCH_SIZE) as i32);
                        o.draw_instances(Self::MAX_BATCH_SIZE);
                    }
                    let ex = i.0.len() % Self::MAX_BATCH_SIZE;
                    if ex > 0 {
                        self.picking_handler.instances_uniform.raw_array_mat4(&i.1[(i.0.len() - ex) * 16 ..]);
                        self.picking_handler.id_uniform.int(acc_vec.len() as i32 + ((i.0.len() / Self::MAX_BATCH_SIZE) * Self::MAX_BATCH_SIZE) as i32);
                        o.draw_instances(ex);
                    }
                }
                acc_vec.extend(&i.0);
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
            acc_vec[id - 1].clone_weak()
        } else {
            Handle::EMPTY
        }
    }
    
    fn pre_draw(model: &Handle<MultiPartModel>, group: &mut (Vec<Handle<ObjectData>>, Vec<f32>, Vec<i32>)) {
        if model.dirty() {
            group.1.clear();
            group.2.clear();
            let mut once = true;
            for t in &mut group.0 {
                if t.present() {
                    if once {
                        once = false;
                        t.get_mut::<Handle<MultiPartModel>>().clear_dirty();
                    }
                    t.clear_dirty();
                    group.1.extend(Vec::<f32>::from(Mat4::from(t.get::<Transform>())));
                    group.2.push(*t.get::<i32>());
                }
            }
        } else {
            for (i, t) in group.0.iter_mut().enumerate() {
                if t.dirty() {
                    Mat4::from(t.get::<Transform>()).raw_copy(&mut group.1[16 * i .. 16 * i + 16]);
                    group.2[i] = *t.get::<i32>();
                }
            }
        }
    }
    
    pub fn draw(&mut self) {
        for (o, i) in self.instances.iter_mut() {
            if o.present() {
                Self::pre_draw(o, i);
                if i.0.len() == 0 {
                    continue;
                }
                if i.0.len() < Self::MAX_BATCH_SIZE {
                    self.instances_uniform.raw_array_mat4(&i.1);
                    self.flags_uniform.array_int(&i.2);
                    o.draw_instances(i.0.len());
                } else {
                    for batch in 0 .. i.0.len() / Self::MAX_BATCH_SIZE {
                        self.instances_uniform.raw_array_mat4(&i.1[batch * Self::MAX_BATCH_SIZE * 16 .. (batch * Self::MAX_BATCH_SIZE + Self::MAX_BATCH_SIZE) * 16]);
                        self.flags_uniform.array_int(&i.2[batch * Self::MAX_BATCH_SIZE .. batch * Self::MAX_BATCH_SIZE + Self::MAX_BATCH_SIZE]);
                        o.draw_instances(Self::MAX_BATCH_SIZE);
                    }
                    let ex = i.0.len() % Self::MAX_BATCH_SIZE;
                    if ex > 0 {
                        self.instances_uniform.raw_array_mat4(&i.1[(i.0.len() - ex) * 16 ..]);
                        self.flags_uniform.array_int(&i.2[(i.0.len() - ex) ..]);
                        o.draw_instances(ex);
                    }
                }
            }
        }
    }
    
    pub fn iter_instances_mut(&mut self) -> impl Iterator<Item = &mut Handle<ObjectData>> {
        self.instances.iter_mut().flat_map(|(_, v)| &mut v.0)
    }
    
    pub fn debug(&self) {
        for (k, v) in &self.instances {
            dbg!(k, v.0.len());
        }
    }
}