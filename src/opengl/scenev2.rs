use std::collections::HashMap;
use crate::maths::matrix::{Mat4, Matrix};
use crate::maths::transform::Transform;
use crate::opengl::objectv2::MultiPartModel;
use crate::opengl::shader::ShaderProgram;
use crate::opengl::uniform::Uniform;
use crate::other::handles::Handle;
use crate::other::resource_manager::ResourceManager;

pub struct ObjectData {
    pub model: Handle<MultiPartModel>,
    pub transform: Transform
}

pub struct Scene {
    camera: Transform,
    camera_uniform: Uniform,
    projection_uniform: Uniform,
    models: Vec<Handle<MultiPartModel>>,
    instances: HashMap<Handle<MultiPartModel>, (Vec<Handle<ObjectData>>, Vec<f32>)>,
    instances_uniform: Uniform,
}

impl Scene {
    pub const MAX_BATCH_SIZE: usize = 128;
    
    pub fn new(program: ShaderProgram) -> Self {
        Self {
            camera: Transform::default(),
            camera_uniform: program.uniform("camera"),
            projection_uniform: program.uniform("projection"),
            models: Vec::new(),
            instances: HashMap::new(),
            instances_uniform: program.uniform("object"),
        }
    }
    
    pub fn set_projection(&mut self, fov: f32, aspect_ratio: f32) {
        self.projection_uniform.mat4(Matrix::projection(fov.to_radians(), aspect_ratio, 0.01, 1000.));
    }
    
    pub fn set_camera(&mut self, camera: Transform) {
        self.camera_uniform.mat4(camera.as_view_matrix());
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
    
    pub fn spawn_object(&mut self, model: &Handle<MultiPartModel>, transform: Transform) -> Handle<ObjectData> {
        let (instances, raw) = self.instances.entry(model.clone_weak()).or_default();
        let instance = Handle::new_strong(ObjectData {
            model: model.clone_weak(),
            transform,
        });
        let out = instance.clone_weak();
        instances.push(instance);
        raw.extend(Vec::<f32>::from(Mat4::from(transform)));
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
    
    pub fn draw(&mut self) {
        for (o, i) in self.instances.iter_mut() {
            if let Some(m) = o.get() {
                if o.dirty() || i.0.iter_mut().any(|h| h.dirty()) {
                    i.1.clear();
                    let mut once = true;
                    for t in &mut i.0 {
                        if once {
                            if let Some(ObjectData { model, .. }) = t.get_mut() {
                                model.clear_dirty();
                            }
                        }
                        t.clear_dirty();
                        if let Some(ObjectData { transform, .. }) = t.get() {
                            i.1.extend(Vec::<f32>::from(Mat4::from(*transform)));
                        }
                    }
                }
                if i.0.len() < Self::MAX_BATCH_SIZE {
                    self.instances_uniform.raw_array_mat4(&i.1);
                    m.draw_instances(i.0.len());
                } else {
                    for batch in 0 .. i.0.len() / Self::MAX_BATCH_SIZE {
                        self.instances_uniform.raw_array_mat4(&i.1[batch * Self::MAX_BATCH_SIZE * 16 .. (batch * Self::MAX_BATCH_SIZE + Self::MAX_BATCH_SIZE) * 16]);
                        m.draw_instances(Self::MAX_BATCH_SIZE);
                    }
                    let ex = i.0.len() % Self::MAX_BATCH_SIZE;
                    if ex > 0 {
                        self.instances_uniform.raw_array_mat4(&i.1[(i.0.len() - ex) * 16 ..]);
                        m.draw_instances(ex);
                    }
                }
            }
        }
    }
}