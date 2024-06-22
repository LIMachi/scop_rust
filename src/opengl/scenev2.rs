use std::collections::{HashMap, HashSet};
use crate::maths::matrix::{Mat4, Matrix};
use crate::maths::transform::Transform;
use crate::opengl::lights::Light;
use crate::opengl::objectv2::MultiPartModel;
use crate::opengl::shader::ShaderProgram;
use crate::opengl::uniform::Uniform;
use crate::other::resource_manager::ResourceManager;

//possible visibility changes:
//camera / projection change: recalculate entire frustrum visibility of camera (but not the lights)
//object spawn/change: only recalculate visibility of this object, recalculate lights that saw it previously or can see it now
//object despawn: only recalculate lights if needed, will not affect the visibility of other objects
//light spawn/move: recalculate this light
//light despawn: remove refs to this light in shader

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum ChangeEvents {
    FrustrumUpdate, //used when the camera transform or the projection changed since last frame
    ObjectUpdate(usize), //recheck visibility of this object by camera or lights (frustrum + re render)
    ObjectDespawn(usize), //re render without frustrum (should remove all instances of ObjectUpdate with the same id on insert)
    LightUpdate(usize), //re render shadow map of light
}

#[derive(Debug, Copy, Clone)]
pub struct ObjectData {
    pub model: usize,
    pub transform: Transform
}

#[derive(Debug)]
pub struct Scene {
    camera: Transform,
    camera_uniform: Uniform,
    projection_uniform: Uniform,
    models: HashMap<usize, MultiPartModel>,
    next_model_id: usize,
    objects: HashMap<usize, ObjectData>,
    next_object_id: usize,
    instances: HashMap<usize, (HashMap<usize, Transform>, Vec<f32>)>, //mapping: model -> (id -> transform, raw transforms)
    instances_uniform: Uniform,
    lights: HashMap<usize, Light>,
    next_light_id: usize,
    lights_uniform: Uniform,
    light_count_uniform: Uniform,
    changes: HashSet<ChangeEvents>,
}

impl Scene {
    pub const MAX_BATCH_SIZE: usize = 128;
    
    pub fn new(program: ShaderProgram) -> Self {
        Self {
            camera: Transform::default(),
            camera_uniform: program.uniform("camera"),
            projection_uniform: program.uniform("projection"),
            models: HashMap::new(),
            next_model_id: 0,
            objects: HashMap::new(),
            next_object_id: 0,
            instances: HashMap::new(),
            instances_uniform: program.uniform("object"),
            lights: HashMap::new(),
            next_light_id: 0,
            lights_uniform: program.uniform("lights"),
            light_count_uniform: program.uniform("light_count"),
            changes: HashSet::new()
        }
    }
    
    pub fn set_projection(&mut self, fov: f32, aspect_ratio: f32) {
        self.projection_uniform.mat4(Matrix::projection(fov.to_radians(), aspect_ratio, 0.01, 1000.));
        self.changes.insert(ChangeEvents::FrustrumUpdate);
    }
    
    pub fn set_camera(&mut self, camera: Transform) {
        self.camera_uniform.mat4(camera.as_view_matrix());
        self.camera = camera;
        self.changes.insert(ChangeEvents::FrustrumUpdate);
    }
    
    pub fn get_camera(&self) -> Transform { self.camera }

    pub fn load_model<S: Into<String>>(&mut self, resources: &mut ResourceManager, name: S) -> usize {
        if let Some(po) = resources.load_object(name).get() {
            self.models.insert(self.next_model_id, MultiPartModel::new(resources, &po));
            self.next_model_id += 1;
            self.next_model_id - 1
        } else {
            0
        }
    }
    
    pub fn spawn_object(&mut self, model: usize, transform: Transform) -> usize {
        if self.models.contains_key(&model) {
            let id = self.next_object_id;
            self.next_object_id += 1;
            self.objects.insert(id, ObjectData {
                model,
                transform
            });
            self.changes.insert(ChangeEvents::ObjectUpdate(id));
            id
        } else {
            0
        }
    }
    
    pub fn spawn_light(&mut self, light: Light) -> usize {
        let id = self.next_light_id;
        self.next_light_id += 1;
        self.lights.insert(id, light);
        self.changes.insert(ChangeEvents::LightUpdate(id));
        id
    }
    
    pub fn get_object(&self, object: usize) -> Option<ObjectData> {
        self.objects.get(&object).copied()
    }
    
    pub fn get_light(&self, light: usize) -> Option<Light> {
        self.lights.get(&light).copied()
    }
    
    pub fn set_object(&mut self, object: usize, data: ObjectData) {
        if !self.changes.contains(&ChangeEvents::ObjectDespawn(object)) {
            if let Some(obj) = self.objects.get_mut(&object) {
                *obj = data;
                self.changes.insert(ChangeEvents::ObjectUpdate(object));
            }
        }
    }
    
    pub fn set_light(&mut self, light: usize, data: Light) {
        if let Some(l) = self.lights.get_mut(&light) {
            *l = data;
            self.changes.insert(ChangeEvents::LightUpdate(light));
        }
    }
    
    pub fn despawn_object(&mut self, object: usize) {
        if self.objects.contains_key(&object) {
            self.changes.remove(&ChangeEvents::ObjectUpdate(object));
            self.changes.insert(ChangeEvents::ObjectDespawn(object));
        }
    }
    
    pub fn despawn_light(&mut self, light: usize) {
        self.lights.remove(&light);
    }
    
    pub fn update(&mut self) {
        let mut batches = HashSet::new();
        for change in &self.changes {
            match change {
                ChangeEvents::FrustrumUpdate => {}
                ChangeEvents::ObjectUpdate(id) => {
                    //for now, no frustrum, we only collect the changes on a batch kind and rebuild it entirely
                    if let Some(obj) = self.objects.get(&id) {
                        if self.models.contains_key(&obj.model) {
                            batches.insert(obj.model);
                            let (instances, _) = self.instances.entry(obj.model).or_default();
                            instances.insert(*id, obj.transform);
                        }
                    }
                }
                ChangeEvents::ObjectDespawn(id) => {
                    if let Some(obj) = self.objects.remove(&id) {
                        if self.models.contains_key(&obj.model) {
                            let mut clear = false;
                            if let Some((instances, _)) = self.instances.get_mut(&obj.model) {
                                instances.remove(id);
                                if instances.len() > 0 {
                                    batches.insert(obj.model);
                                } else {
                                    clear = true;
                                }
                            }
                            if clear {
                                self.instances.remove(&obj.model);
                            }
                        }
                    }
                }
                ChangeEvents::LightUpdate(id) => {}
            }
        }
        for batch in &batches {
            if let Some((instances, v)) = self.instances.get_mut(batch) {
                v.clear();
                for instance in instances {
                    v.extend(Vec::<f32>::from(Mat4::from(*instance.1)));
                }
            }
        }
    }
    
    pub fn draw(&mut self) {
        self.update();
        for (o, i) in &self.instances {
            if let Some(o) = self.models.get(o) {
                if i.0.len() < Self::MAX_BATCH_SIZE {
                    self.instances_uniform.raw_array_mat4(&i.1);
                    o.draw_instances(i.0.len());
                } else {
                    for batch in 0 .. i.0.len() / Self::MAX_BATCH_SIZE {
                        self.instances_uniform.raw_array_mat4(&i.1[batch * Self::MAX_BATCH_SIZE * 16 .. (batch * Self::MAX_BATCH_SIZE + Self::MAX_BATCH_SIZE) * 16]);
                        o.draw_instances(Self::MAX_BATCH_SIZE);
                    }
                    let ex = i.0.len() % Self::MAX_BATCH_SIZE;
                    if ex > 0 {
                        self.instances_uniform.raw_array_mat4(&i.1[(i.0.len() - ex) * 16 ..]);
                        o.draw_instances(ex);
                    }
                }
            }
        }
    }
}