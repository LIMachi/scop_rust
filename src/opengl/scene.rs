use std::collections::HashSet;
use std::os::raw::c_void;
use gl::types::GLint;
use crate::maths::matrix::{Mat4, Matrix};
use crate::maths::transform::Transform;
use crate::opengl::enums::Shaders;
use crate::opengl::main_shader::MainShader;
use crate::opengl::safe_calls;
use crate::opengl::shader::{ShaderProgram, ShaderProgramBuilder};
use crate::opengl::uniform::Uniform;
use crate::other::itermap::IterMap;
use crate::other::resource_manager::ResourceManager;

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
pub struct Batch {
    pub size: usize,
    pub mat: [f32; MAX_BATCH_SIZE * 16],
    pub rf: [i32; MAX_BATCH_SIZE],
}

impl Default for Batch {
    fn default() -> Self {
        Self {
            size: 0,
            mat: [0f32; MAX_BATCH_SIZE * 16],
            rf: [0i32; MAX_BATCH_SIZE],
        }
    }
}

impl Batch {
    pub fn bake(&self, shader: &MainShader) {
        shader.object.raw_array_mat4(&self.mat);
        shader.flags.array_int(&self.rf);
    }
}

#[derive(Debug)]
pub struct ObjectData {
    pub transform: Transform,
    pub raw_mat: [f32; 16],
    pub flags: i32,
    pub visible: bool,
}

impl ObjectData {
    pub fn with_flags(mut self, flags: i32) -> Self {
        self.flags = flags;
        self
    }
}

impl From<Transform> for ObjectData {
    fn from(value: Transform) -> Self {
        Self {
            raw_mat: Mat4::from(&value).raw_array(),
            transform: value,
            flags: 0,
            visible: true
        }
    }
}

#[derive(Debug)]
pub struct Scene {
    camera: Transform,
    projection: Mat4,
    next_instance_id: usize,
    instances: IterMap<usize, IterMap<usize, ObjectData>>,
    shader: MainShader,
    picking_handler: PickingHandler,
    batch_storage: Vec<Batch>
}

impl Scene {
    pub fn new(shader: ShaderProgram) -> Self {
        Self {
            camera: Transform::default(),
            projection: Mat4::identity(),
            next_instance_id: 0,
            instances: IterMap::new(),
            shader: MainShader::new(shader),
            picking_handler: PickingHandler::new(),
            batch_storage: Vec::new()
        }
    }
    
    pub fn set_projection(&mut self, fov: f32, aspect_ratio: f32) {
        let proj = Matrix::projection(fov.to_radians(), aspect_ratio, 0.01, 1000.);
        self.shader.program.set_active();
        self.shader.projection.mat4(proj);
        self.projection = proj;
        self.picking_handler.shader.set_active();
        self.picking_handler.projection_uniform.mat4(proj);
    }
    
    pub fn set_camera(&mut self, camera: Transform) {
        let mat = camera.as_view_matrix();
        self.shader.program.set_active();
        self.shader.camera.mat4(mat);
        self.picking_handler.shader.set_active();
        self.picking_handler.camera_uniform.mat4(mat);
        self.camera = camera;
    }
    
    pub fn get_camera(&self) -> Transform { self.camera }
    
    pub fn get_projection(&self) -> Mat4 { self.projection }

    pub fn spawn_object(&mut self, model: usize, data: ObjectData) -> usize {
        let v = self.instances.get_mut_or_insert(&model, |_| IterMap::new());
        v.insert(self.next_instance_id, data);
        self.next_instance_id += 1;
        self.next_instance_id - 1
    }
    
    pub fn despawn_object(&mut self, id: usize) {
        let mut check = None;
        for (k, v) in self.instances.iter_mut() {
            if let Some(_) = v.remove(&id) {
                check = Some(*k);
                break;
            }
        }
        if let Some(c) = check {
            if self.instances.get(&c).unwrap().len() == 0 {
                self.instances.remove(&c);
            }
        }
    }

    fn extract_batches<'a>(storage: &mut Vec<Batch>, instances: impl Iterator<Item = &'a ObjectData>) {
        storage.clear();
        storage.push(Batch::default());
        for ObjectData { transform, raw_mat, flags, visible } in instances {
            let batch = {
                if storage.last().unwrap().size == MAX_BATCH_SIZE {
                    storage.push(Batch::default());
                }
                storage.last_mut().unwrap()
            };
            for i in 0..16 {
                batch.mat[batch.size * 16 + i] = raw_mat[i];
            }
            batch.rf[batch.size] = *flags;
            batch.size += 1;
        }
    }

    pub fn pick(&mut self, resources: &ResourceManager, pixel_x: usize, pixel_y: usize, set: Option<&HashSet<usize>>) -> Option<usize> {
        let mut acc_vec = Vec::new();
        safe_calls::clear_screen();
        self.picking_handler.shader.set_active();
        for (model, instances) in self.instances.iter_mut() {
            if let Some(mpm) = resources.get_multipart_model(*model) {
                if let Some(set) = set {
                    Self::extract_batches(&mut self.batch_storage, instances.iter().filter_map(|(id, v)| {
                        if v.visible && set.contains(id) {
                            Some(v)
                        } else {
                            None
                        }
                    }));
                } else {
                    Self::extract_batches(&mut self.batch_storage, instances.iter_values().filter(|v| v.visible));
                }
                let l = acc_vec.len();
                for (c, Batch { size, mat, rf }) in self.batch_storage.iter().enumerate() {
                    // self.picking_handler.instances_uniform.raw_array_mat4(&mat[0..*size * 16]);
                    self.picking_handler.id_uniform.int((l + c * MAX_BATCH_SIZE) as i32);
                    mpm.draw_instances(*size, None);
                }
                acc_vec.extend(instances.iter().map(|(k, _)| *k));
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
        if id > 0 && id <= acc_vec.len() {
            Some(acc_vec[id - 1])
        } else {
            None
        }
    }
    
    pub fn draw(&mut self, resources: &ResourceManager, set: Option<&HashSet<usize>>) {
        self.shader.program.set_active();
        for (model, instances) in self.instances.iter_mut() {
            if let Some(mpm) = resources.get_multipart_model(*model) {
                if let Some(set) = set {
                    Self::extract_batches(&mut self.batch_storage, instances.iter().filter_map(|(id, v)| {
                        if v.visible && set.contains(id) {
                            Some(v)
                        } else {
                            None
                        }
                    }));
                } else {
                    Self::extract_batches(&mut self.batch_storage, instances.iter_values().filter(|v| v.visible));
                }
                for Batch { size, mat, rf } in &self.batch_storage {
                    self.shader.object.raw_array_mat4(&mat[0..*size * 16]);
                    self.shader.flags.array_int(&rf[0..*size]);
                    mpm.draw_instances(*size, Some(&self.shader));
                }
            }
        }
    }

    pub fn run_on_instance<F: FnMut(usize, usize, &mut ObjectData)>(&mut self, id: usize, mut runner: F) {
        for (model, v) in self.instances.iter_mut() {
            if let Some(v) = v.get_mut(&id) {
                runner(*model, id, v);
                return;
            }
        }
    }

    pub fn run_on_instances<F: FnMut(usize, usize, &mut ObjectData)>(&mut self, mut runner: F) {
        for (model, v) in self.instances.iter_mut() {
            for (id, v) in v.iter_mut() {
                runner(*model, *id, v);
            }
        }
    }
}