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
use crate::other::handles::Handle;
use crate::other::resource_manager::ResourceManager;

#[derive(Debug)]
pub struct ObjectData {
    pub model: Handle<MultiPartModel>,
    pub transform: Transform,
    pub flags: i32,
}

impl Clone for ObjectData {
    fn clone(&self) -> Self {
        Self {
            model: self.model.clone_weak(),
            transform: self.transform,
            flags: self.flags
        }
    }
}

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

pub struct Scene {
    camera: Transform,
    camera_uniform: Uniform,
    projection_uniform: Uniform,
    models: Vec<Handle<MultiPartModel>>,
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
            models: Vec::new(),
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

    pub fn load_model<S: Into<String>>(&mut self, resources: &mut ResourceManager, name: S) -> Handle<MultiPartModel> {
        if let Some(po) = resources.load_object(name).get() {
            let h = Handle::new_strong(MultiPartModel::new(resources, &po));
            let out = h.clone_weak();
            self.models.push(h);
            out
        } else {
            Handle::EMPTY
        }
    }
    
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
        if let Some(t) = object.get() {
            if let Some(i) = self.instances.get_mut(&t.model) {
                i.0.retain(|v| v != &object);
            }
        }
        object.get_mut().map(|t| t.model.get_mut()); //trick to make model dirty and force rebatching
    }
    
    pub fn pick(&mut self, pixel_x: usize, pixel_y: usize) -> Handle<ObjectData> {
        let mut acc_vec = Vec::new();
        safe_calls::clear_screen();
        self.picking_handler.shader.set_active();
        for (o, i) in self.instances.iter_mut() {
            if let Some(m) = o.get() {
                Self::pre_draw(o, i);
                if i.0.len() < Self::MAX_BATCH_SIZE {
                    self.picking_handler.instances_uniform.raw_array_mat4(&i.1);
                    self.picking_handler.id_uniform.int(0);
                    m.draw_instances(i.0.len());
                } else {
                    for batch in 0 .. i.0.len() / Self::MAX_BATCH_SIZE {
                        self.picking_handler.instances_uniform.raw_array_mat4(&i.1[batch * Self::MAX_BATCH_SIZE * 16 .. (batch * Self::MAX_BATCH_SIZE + Self::MAX_BATCH_SIZE) * 16]);
                        self.picking_handler.id_uniform.int((batch * Self::MAX_BATCH_SIZE) as i32);
                        m.draw_instances(Self::MAX_BATCH_SIZE);
                    }
                    let ex = i.0.len() % Self::MAX_BATCH_SIZE;
                    if ex > 0 {
                        self.picking_handler.instances_uniform.raw_array_mat4(&i.1[(i.0.len() - ex) * 16 ..]);
                        self.picking_handler.id_uniform.int(((i.0.len() / Self::MAX_BATCH_SIZE) * Self::MAX_BATCH_SIZE) as i32);
                        m.draw_instances(ex);
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
    
    fn pre_draw(o: &Handle<MultiPartModel>, i: &mut (Vec<Handle<ObjectData>>, Vec<f32>, Vec<i32>)) {
        if o.dirty() || i.0.iter_mut().any(|h| h.dirty()) {
            i.1.clear();
            i.2.clear();
            let mut once = true;
            for t in &mut i.0 {
                if once {
                    if let Some(ObjectData { model, .. }) = t.get_mut() {
                        model.clear_dirty();
                    }
                }
                t.clear_dirty();
                if let Some(ObjectData { transform, flags, .. }) = t.get() {
                    i.1.extend(Vec::<f32>::from(Mat4::from(*transform)));
                    i.2.push(*flags);
                }
            }
        }
    }
    
    pub fn draw(&mut self) {
        for (o, i) in self.instances.iter_mut() {
            if let Some(m) = o.get() {
                Self::pre_draw(o, i);
                if i.0.len() < Self::MAX_BATCH_SIZE {
                    self.instances_uniform.raw_array_mat4(&i.1);
                    self.flags_uniform.array_int(&i.2);
                    m.draw_instances(i.0.len());
                } else {
                    for batch in 0 .. i.0.len() / Self::MAX_BATCH_SIZE {
                        self.instances_uniform.raw_array_mat4(&i.1[batch * Self::MAX_BATCH_SIZE * 16 .. (batch * Self::MAX_BATCH_SIZE + Self::MAX_BATCH_SIZE) * 16]);
                        self.flags_uniform.array_int(&i.2[batch * Self::MAX_BATCH_SIZE .. batch * Self::MAX_BATCH_SIZE + Self::MAX_BATCH_SIZE]);
                        m.draw_instances(Self::MAX_BATCH_SIZE);
                    }
                    let ex = i.0.len() % Self::MAX_BATCH_SIZE;
                    if ex > 0 {
                        self.instances_uniform.raw_array_mat4(&i.1[(i.0.len() - ex) * 16 ..]);
                        self.flags_uniform.array_int(&i.2[(i.0.len() - ex) ..]);
                        m.draw_instances(ex);
                    }
                }
            }
        }
    }
}