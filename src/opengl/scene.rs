use std::collections::HashMap;
use std::ffi::c_void;
use std::mem::size_of;
use gl::types::{GLint, GLsizei, GLsizeiptr, GLuint};
use crate::maths::matrix::{Mat4, Matrix};
use crate::maths::transform::Transform;
use crate::maths::vector::Vec3;
use crate::opengl::enums::Shaders;
use crate::opengl::object::Model;
use crate::opengl::part::ObjectPart;
use crate::opengl::safe_calls;
use crate::opengl::shader::{Drawable, ShaderProgram, ShaderProgramBuilder};
use crate::opengl::uniform::Uniform;
use crate::other::resource_manager::ResourceManager;

pub struct Scene {
    pub objects_program: ShaderProgram,
    pub lights_program: ShaderProgram,
    pub test_program: ShaderProgram,
    pub debug_program: ShaderProgram,
    pub models: Vec<Model>,
    pub camera: Transform,
    pub camera_uniform: Uniform, //only updated when camera is modified
    pub projection_uniform: Uniform, //only updated when aspect ratio changes (or later, when fov changes)
    pub objects: HashMap<usize, (Transform, usize)>,
    pub next_object_id: usize, //keep track of the next id that will be used by the object map (when spawning a new object, this value will be used as the id of the spawned object and increased)
    pub object_transform_uniform: Uniform, //updated when rendering more than 1 object
    pub object_flags_uniform: Uniform, //updated if 2 objects have different flags or when debug-ing Lights
    pub lights: HashMap<usize, (Transform, Vec3)>,
    pub next_light_id: usize, //same id system as the objects
    pub lights_vao: GLuint,
    pub lights_pos_vbo: GLuint,
    pub lights_color_vbo: GLuint,
    pub light_count_uniform: Uniform,
    pub light_array_uniform: Uniform,
    pub lights_camera_uniform: Uniform,
    pub lights_projection_uniform: Uniform,
    pub depth_map_fbo: GLuint,
    pub depth_map: GLuint,
    pub rebuild_flags: usize,
    pub debug_vao: GLuint,
}

impl Scene {
    pub fn new(objects_program: ShaderProgram, lights_program: ShaderProgram, initial_camera_transform: Transform, fov: f32, aspect_ratio: f32) -> Self {
        objects_program.set_active();
        let projection_uniform = objects_program.uniform("proj");
        let projection = Matrix::projection(fov.to_radians(), aspect_ratio, 0.1, 10000.);
        projection_uniform.mat4(projection);
        let lights_projection_uniform = lights_program.uniform("proj");
        let camera_uniform = objects_program.uniform("camera");
        let view = initial_camera_transform.as_view_matrix();
        camera_uniform.mat4(view);
        let lights_camera_uniform = lights_program.uniform("camera");
        lights_program.set_active();
        lights_projection_uniform.mat4(projection);
        lights_camera_uniform.mat4(view);
        let mut lights_vao = 0;
        let mut lights_pos_vbo = 0;
        let mut lights_color_vbo = 0;
        let mut depth_map_fbo = 0;
        let mut depth_map = 0;
        let mut debug_vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut lights_vao);
            gl::BindVertexArray(lights_vao);
            gl::GenBuffers(1, &mut lights_pos_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, lights_pos_vbo);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, ObjectPart::VEC3_SIZE as GLsizei, 0 as *const _);

            gl::GenBuffers(1, &mut lights_color_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, lights_color_vbo);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, ObjectPart::VEC3_SIZE as GLsizei, 0 as *const _);
            
            gl::GenFramebuffers(1, &mut depth_map_fbo); //generate a new frame buffer to store the output of a shader (instead of rendering to the default frame buffer, the one we swap every frame to refresh the screen)
            
            //setup the depth buffer texture (so we can store it and reuse it in another shader latter on)
            gl::GenTextures(1, &mut depth_map);
            gl::BindTexture(gl::TEXTURE_2D, depth_map);
            //declare the texture as 1024*1024 pixels using the same format as the default depth texture used by the shaders
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::DEPTH_COMPONENT as GLint, 1024, 1024, 0, gl::DEPTH_COMPONENT, gl::FLOAT, 0 as *const _);
            //setup the default filtering (zoom effects) and tilling effects
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
            
            gl::BindFramebuffer(gl::FRAMEBUFFER, depth_map_fbo);
            //now bind the frame buffer to this texture
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::TEXTURE_2D, depth_map, 0);
            gl::DrawBuffer(gl::NONE); //explicitly discard the color buffers (since this pass is only used to generate a depth map)
            gl::ReadBuffer(gl::NONE);

            gl::GenVertexArrays(1, &mut debug_vao);
            let mut t = 0;
            gl::BindVertexArray(debug_vao);
            gl::GenBuffers(1, &mut t);
            gl::BindBuffer(gl::ARRAY_BUFFER, t);
            let fsqv: [f32; 24] = [
                -1.,  1.,  0., 1.,
                -1., -1.,  0., 0.,
                1., -1.,  1., 0.,
                -1.,  1.,  0., 1.,
                1., -1.,  1., 0.,
                1.,  1.,  1., 1.
            ];
            gl::BufferData(gl::ARRAY_BUFFER, size_of::<[f32; 24]>() as GLsizeiptr, fsqv.as_ptr() as *const c_void, gl::STATIC_DRAW);
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, size_of::<[f32; 4]>() as GLsizei, 0 as *const _);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, size_of::<[f32; 4]>() as GLsizei, size_of::<[f32; 2]>() as *const _);
        }
        Self {
            models: Vec::new(),
            camera: initial_camera_transform,
            camera_uniform,
            projection_uniform,
            objects: HashMap::new(),
            next_object_id: 0,
            object_transform_uniform: objects_program.uniform("object"),
            object_flags_uniform: objects_program.uniform("flags"),
            lights: HashMap::new(),
            next_light_id: 0,
            lights_vao,
            lights_pos_vbo,
            lights_color_vbo,
            light_count_uniform: objects_program.uniform("light_count"),
            light_array_uniform: objects_program.uniform("Lights"),
            lights_camera_uniform,
            objects_program,
            rebuild_flags: 0,
            lights_program,
            lights_projection_uniform,
            depth_map_fbo,
            depth_map,
            debug_vao,
            test_program: ShaderProgramBuilder::default().add_shader(Shaders::Vertex, include_str!("lights/lights.vert")).add_shader(Shaders::Fragment, include_str!("lights/lights.frag")).build().unwrap(),
            debug_program: ShaderProgramBuilder::default().add_shader(Shaders::Vertex, include_str!("lights/debug.vert")).add_shader(Shaders::Fragment, include_str!("lights/debug.frag")).build().unwrap()
        }
    }
    
    pub fn update_projection(&self, fov: f32, aspect_ratio: f32) {
        let projection = Matrix::projection(fov.to_radians(), aspect_ratio, 0.1, 10000.);
        self.objects_program.set_active();
        self.projection_uniform.mat4(projection);
        self.lights_program.set_active();
        self.lights_projection_uniform.mat4(projection);
    }
    
    pub fn load_model<S: Into<String>>(&mut self, resources: &mut ResourceManager, name: S) -> usize {
        if let Some(po) = resources.load_object(name).get() {
            let mut model = Model::new(resources, &po);
            model.bake(&self.objects_program);
            self.models.push(model);
            self.models.len()
        } else {
            0
        }
    }
    
    pub fn spawn_object(&mut self, model: usize, transform: Transform) -> usize {
        if model > 0 && model <= self.models.len() {
            self.objects.insert(self.next_object_id, (transform, model - 1));
            self.next_object_id += 1;
            self.next_object_id - 1
        } else {
            0
        }
    }
    
    pub fn get_object_mut(&mut self, id: usize) -> Option<&mut Transform> {
        self.objects.get_mut(&id).map(|v| &mut v.0)
    }
    
    pub fn despawn_object(&mut self, id: usize) -> bool { self.objects.remove(&id).is_some() }
    
    pub fn spawn_light(&mut self, transform: Transform, color: Vec3) -> usize {
        self.lights.insert(self.next_light_id, (transform, color));
        self.rebuild_flags |= 2;
        self.next_light_id += 1;
        self.next_light_id - 1
    }
    
    pub fn get_light_mut(&mut self, id: usize) -> Option<&mut (Transform, Vec3)> {
        self.rebuild_flags |= 2;
        self.lights.get_mut(&id)
    }
    
    pub fn despawn_light(&mut self, id: usize) -> bool {
        self.rebuild_flags |= 2;
        self.lights.remove(&id).is_some()
    }
    
    pub fn get_camera_mut(&mut self) -> &mut Transform {
        self.rebuild_flags |= 1;
        &mut self.camera
    }
    
    //debug the camera as if it was a spotlight
    pub fn directional_light_depth_map(&mut self) {
        self.camera_uniform.mat4(self.camera.as_view_matrix());
        self.projection_uniform.mat4(Mat4::orthographic(1024., 1., 0.1, 1000.));
        self.test_program.set_active();
        let (pw, ph) = safe_calls::get_size();
        safe_calls::resize(1024, 1024);
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.depth_map_fbo);
            gl::Clear(gl::DEPTH_BUFFER_BIT);
            //setup
            for (_, (tr, i)) in &self.objects {
                if *i < self.models.len() {
                    self.models[*i].bind();
                    self.object_flags_uniform.int(self.models[*i].render_flags);
                    self.object_transform_uniform.mat4(Mat4::from(*tr));
                    self.models[*i].draw();
                }
            }
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
        safe_calls::resize(pw, ph);
        self.debug_program.set_active();
        safe_calls::clear_screen();
        self.debug_program.uniform("tex").int(self.depth_map as i32);
        unsafe {
            gl::BindVertexArray(self.debug_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 24);
        }
    }
    
    pub fn draw(&mut self) {
        if (self.rebuild_flags & 1) == 1 {
            let view = self.camera.as_view_matrix();
            self.objects_program.set_active();
            self.camera_uniform.mat4(view);
            self.lights_program.set_active();
            self.lights_camera_uniform.mat4(view);
        }
        if (self.rebuild_flags & 2) == 2 {
            let mut weave = Vec::with_capacity(self.lights.len() * 2);
            let mut pos = Vec::with_capacity(self.lights.len());
            let mut color = Vec::with_capacity(self.lights.len());
            for (_, (tr, c)) in &self.lights {
                let p = <[f32; 3]>::from(tr.pos);
                let c = <[f32; 3]>::from(*c);
                weave.push(p);
                weave.push(c);
                pos.push(p);
                color.push(c);
            }
            self.lights_program.set_active();
            unsafe {
                gl::BindVertexArray(self.lights_vao);
                gl::BindBuffer(gl::ARRAY_BUFFER, self.lights_pos_vbo);
                gl::BufferData(gl::ARRAY_BUFFER, (ObjectPart::VEC3_SIZE * pos.len()) as GLsizeiptr, pos.as_ptr() as *const c_void, gl::STATIC_DRAW);
                gl::BindBuffer(gl::ARRAY_BUFFER, self.lights_color_vbo);
                gl::BufferData(gl::ARRAY_BUFFER, (ObjectPart::VEC3_SIZE * color.len()) as GLsizeiptr, color.as_ptr() as *const c_void, gl::STATIC_DRAW);
            }
            self.objects_program.set_active();
            self.light_count_uniform.int(pos.len() as i32);
            self.light_array_uniform.array3f(&weave);
        }
        self.objects_program.set_active();
        for (_, (tr, i)) in &self.objects {
            if *i < self.models.len() {
                self.models[*i].bind();
                self.object_flags_uniform.int(self.models[*i].render_flags);
                self.object_transform_uniform.mat4(Mat4::from(*tr));
                self.models[*i].draw();
            }
        }
        self.lights_program.set_active();
        if self.lights_vao != 0 {
            unsafe {
                gl::BindVertexArray(self.lights_vao);
                gl::DrawArrays(gl::POINTS, 0, self.lights.len() as GLsizei);
            }
        }
    }
}