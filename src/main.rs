use std::collections::HashMap;
use std::env;
use std::mem::{size_of, size_of_val};
use std::ops::Add;
use winit::dpi::PhysicalPosition;
use winit::event;
use winit::event::{DeviceEvent, ElementState, Event, MouseButton, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::window::{Fullscreen, WindowBuilder};
use crate::maths::matrix::{Mat4, Matrix};
use crate::maths::quaternion::Quaternion;
use crate::maths::transform::Transform;
use crate::maths::vector::{Vec3, Vector};
use crate::opengl::buffers::{GPUBuffers, VertexType};
use crate::opengl::enums::{RenderMode, Shaders, Side};
use crate::opengl::object::Model;
use crate::opengl::objectv2::MultiPartModel;
use crate::opengl::safe_calls;
use crate::opengl::scenev2::Scene;
use crate::opengl::shader::{Drawable, ShaderProgram, ShaderProgramBuilder};
use crate::other::inputs::Inputs;
use crate::other::resource_manager::ResourceManager;
use crate::other::window;
use crate::other::window::GlWindow;

mod parser;
mod opengl;
mod maths;
mod other;

fn main() {
    if let Some((mut window, event_loop)) = GlWindow::new(WindowBuilder::new()
        .with_title("Scop")
        .with_visible(true)
        .with_fullscreen(Some(Fullscreen::Borderless(None)))
    ) {

        let mut resources = ResourceManager::default();
        resources.register_hints(&["resources", "resources/objs", "resources/materials", "resources/textures", "resources/shaders"]);

        let mut program = ShaderProgram::from_resources(&mut resources, "default").unwrap();
        program.set_active();
        let o1 = Transform::from_look_at(Vec3::default(), Vec3::X);
        let o2 = Transform::from_look_at(Vec3::X * 1., Vec3::Z);
        
        // program.uniform("object").array_mat4(&vec![o1, o2]);
        // program.uniform("projection").mat4(Mat4::projection(80f32.to_radians(), 16./9., 0.001, 1000.));
        // program.uniform("camera").mat4(Transform::from_look_at(Vec3::Z * 2., Vec3::default()).as_view_matrix());

        // let obj42 = resources.load_object("42").unwrap().clone();
        // let mut obj = Object::new(&mut resources, &obj42);

        let mut scene = Scene::new(program);
        
        let model = scene.load_model(&mut resources, "42");
        
        let mut all = Vec::new();
        
        all.push(scene.spawn_object(&model, o1, 0));
        
        //stress test: got >144 fps with ~53k (230*230) instance of "42" rotating on my gtx1070 (uncaped with a single object i get ~2000 fps)
        //>144 fps with 900 (30*30) "dragon" rotating (uses a lot more vertices/calculations, pretty sure there might be 50* more vertices in dragon than in 42)
        for i in 0..330 {
            for j in 0..330 {
                all.push(scene.spawn_object(&model, o2 + Vec3::X * i as f32 + Vec3::Y * j as f32, 0));
            }
        }
        
        scene.set_camera(Transform::from_look_at(Vec3::Z * 10., Vec3::default()));
        scene.set_projection(80., 16./9.);
        
        safe_calls::set_clear_color(0., 0.5, 0.2);
        safe_calls::set_depth_test(true);
        safe_calls::set_cull_face(true);

        let mut count = 0;
        let mut once = true;
        
        let mut mouse_pos = PhysicalPosition {
            x: 0.,
            y: 0.,
        };
        let mut process_picking = false;
        
        event_loop.run(move |event, _target, control_flow| {
                match event {
                    Event::WindowEvent {
                        window_id, event,
                    } if window.id() == window_id => {
                        if event == WindowEvent::CloseRequested {
                            *control_flow = ControlFlow::Exit
                        }
                        if let WindowEvent::CursorMoved { device_id, position, modifiers } = event {
                            mouse_pos = position;
                        }
                        if let WindowEvent::MouseInput { device_id, state, button, modifiers } = event {
                            if state == ElementState::Pressed && button == MouseButton::Left {
                                process_picking = true;
                            }
                        }
                    }
                    Event::MainEventsCleared => {
                        if count < 500 {
                            count += 1;
                            if count == 499 {
                                for p in &all[100..] {
                                    scene.despawn_object(p.clone_weak());
                                }
                                all = all[0..100].to_vec();
                            }
                        }
                        for h in &mut all {
                            h.get_mut().unwrap().transform.rotate_absolute(Vec3::Y, 0.1f32.to_radians());
                        }
                        if process_picking {
                            process_picking = false;
                            let mut t = scene.pick(mouse_pos.x as usize, (safe_calls::get_size().1 as f64 - mouse_pos.y) as usize);
                            if let Some(t) = t.get_mut() {
                                // t.transform.rotate_absolute(Vec3::X, 90f32.to_radians());
                                if t.flags == 0 {
                                    t.flags = 3;
                                } else {
                                    t.flags = 0;
                                }
                            }
                        }
                        safe_calls::clear_screen();
                        scene.draw();
                        window.refresh();
                    }
                    _ => {}
                }
            });
    }
}

/*
fn main() {
    let mut resources = ResourceManager::default();
    resources.register_hints(&["resources", "resources/objs", "resources/materials", "resources/textures", "resources/shaders"]);

    let mut default_texture = "".to_string();
    let mut default_material_lib = "default".to_string();
    let mut default_material = "default".to_string();
    let mut default_frag = "default".to_string();
    let mut default_vert = "default".to_string();
    let mut default_geo = "default".to_string();

    let parsed: Vec<String> = env::args().enumerate().filter_map(|(i, a)| {
        if i == 0 {
            return None;
        }
        if let Some(obj) = resources.load_object(&a) {
            obj.normalize();
            Some(resources.resolve_full_path(a, &["obj"]).unwrap())
        } else {
            if resources.load_texture(&a).is_some() {
                default_texture = a.clone();
            } else if let Some(lib) = resources.load_material_lib(&a) {
                default_material_lib = a.clone();
                if let Some((name, _)) = lib.0.iter().next() {
                    default_material = name.clone();
                }
            } else if a.ends_with(".frag") && resources.load_text(&a).is_some() {
                default_frag = a.clone();
            } else if a.ends_with(".vert") && resources.load_text(&a).is_some() {
                default_vert = a.clone();
            } else if a.ends_with(".geom") && resources.load_text(&a).is_some() {
                default_geo = a.clone();
            }
            None
        }
    }).collect();

    if let Some((mut window, event_loop)) = GlWindow::new(WindowBuilder::new()
        .with_title("Scop")
        .with_visible(true)
        .with_fullscreen(Some(Fullscreen::Borderless(None)))
    ) {
        let mut inputs = Inputs::default_handler();

        safe_calls::set_clear_color(0., 0.5, 0.2);
        safe_calls::set_depth_test(true);
        safe_calls::set_cull_face(true);

        let program = if default_vert == "default" && default_frag == "default" {
            ShaderProgram::from_resources(&mut resources, "default").unwrap()
        } else {
            let mut builder = ShaderProgramBuilder::default();
            if let Some(vert) = resources.load_text(default_vert) {
                builder.add_shader(Shaders::Vertex, vert.as_str());
            }
            if let Some(frag) = resources.load_text(default_frag) {
                builder.add_shader(Shaders::Fragment, frag.as_str());
            }
            if let Some(geo) = resources.load_text(default_geo) {
                builder.add_shader(Shaders::Geometry, geo.as_str());
            }
            builder.build().unwrap()
        };

        let mut scene = Scene::new(program, ShaderProgram::from_resources(&mut resources, "lights").unwrap(), Transform::from_look_at(Vec3::Z * 150., Vector::default()), 80., window.aspect_ratio());

        scene.spawn_light(Transform::default(), Vec3::new(0.5, 0.5, 1.));
        scene.spawn_light(Transform::from_pos(Vec3::new(-100., 10., 10.)), Vec3::new(1., 0.5, 0.5));
        scene.spawn_light(Transform::from_pos(Vec3::new(10., 10., 10.)), Vec3::new(0.5, 1.0, 0.5));

        let mut models = HashMap::new();
        let mut objects = Vec::new();

        for (i, name) in parsed.iter().enumerate() {
            if !models.contains_key(name) {
                models.insert(name, scene.load_model(&mut resources, name));
            }
            objects.push(scene.spawn_object(*models.get(name).unwrap(), Transform::from_look_towards(Vec3::X * i as f32 * 150., -Vec3::X)));
        }

        let mut timer = std::time::Instant::now();

        event_loop.run(move |event, _target, control_flow| {
            match event {
                Event::WindowEvent {
                    window_id, event,
                } if window.id() == window_id => {
                    if window.focused() {
                        if let WindowEvent::KeyboardInput { input, .. } = event {
                            inputs.key_event(input);
                        }
                    }
                    if let WindowEvent::Resized(size) = event {
                        safe_calls::resize(size.width, size.height);
                        scene.update_projection(80., size.height as f32 / size.width as f32);
                    }
                    if event == WindowEvent::CloseRequested {
                        *control_flow = ControlFlow::Exit
                    } else if let WindowEvent::Focused(focused) = event {
                        window.focus(focused);
                    }
                }
                Event::DeviceEvent { event, .. } => {
                    match event {
                        DeviceEvent::MouseMotion { delta: (x, y) } => {
                            if window.focused() {
                                let axis = Vec3::new(-y as f32, -x as f32, 0.).normalize();
                                let strength = (x * x + y * y).sqrt() as f32 / 10.;
                                scene.get_camera_mut().rotate_absolute(axis, strength.to_radians());
                            }
                        }
                        _ => {}
                    }
                }
                Event::MainEventsCleared => {
                    let elapsed = timer.elapsed();
                    if elapsed.as_secs_f64() >= 1. / 60. {
                        timer = std::time::Instant::now();

                        if inputs.pressed().into_iter().count() > 0 {
                            Inputs::apply_to_camera(scene.get_camera_mut(), &inputs, false);
                        }

                        scene.get_object_mut(objects[0]).unwrap().rotate_absolute(Vec3::Y, 0.1f32.to_radians());
                        safe_calls::clear_screen();
                        // scene.directional_light_depth_map();
                        scene.draw();
                        window.refresh();
                    }
                }
                _ => {}
            }
        });
    }
}*/

/*
fn main() {
    let mut resources = ResourceManager::default();
    resources.register_hints(&["resources", "resources/objs", "resources/materials", "resources/textures", "resources/shaders"]);
    
    let mut default_texture = "".to_string();
    let mut default_material = "default".to_string();
    let mut default_frag = "triangle".to_string();
    let mut default_vert = "triangle".to_string();
    
    let parsed: Vec<String> = env::args().enumerate().filter_map(|(i, a)| {
        if i == 0 {
            return None;
        }
        if let Some(obj) = resources.load_object(&a) {
            obj.normalize();
            Some(a)
        } else {
            if resources.load_texture(&a).is_some() {
                default_texture = a.clone();
            } else if resources.load_material_lib(&a).is_some() {
                default_material = a.clone();
            } else if a.contains("frag") && !a.contains("vert") && resources.load_text(&a).is_some() {
                default_frag = a.clone();
            } else if a.contains("vert") && !a.contains("frag") && resources.load_text(&a).is_some() {
                default_vert = a.clone();
            }
            None
        }
    }).collect();
    
    if let Some((ctx, event_loop)) = window::spawn_single_window(WindowBuilder::new()
        .with_title("Scop")
        .with_visible(true)
        .with_fullscreen(Some(Fullscreen::Borderless(None)))
    ) {
        ctx.window().set_cursor_grab(true).unwrap(); //should never produce an error since this project is to be tested on computers
        ctx.window().set_cursor_visible(false);
        let mut program = if default_vert == "triangle" && default_frag == "triangle" {
            ShaderProgram::from_resources(&mut resources, "triangle").unwrap()
        } else {
            let mut builder = ShaderProgramBuilder::default();
            if let Some(vert) = resources.load_text(default_vert) {
                builder.add_shader(Shaders::Vertex, vert.as_str());
            }
            if let Some(frag) = resources.load_text(default_frag) {
                builder.add_shader(Shaders::Fragment, frag.as_str());
            }
            builder.build().unwrap()
        };
        let mut objects: Vec<Model> = parsed.iter().enumerate().map(|(i, s)| {
            let t = resources.load_object(s).unwrap().clone();
            let mut out = Model::new(&mut resources, &t);
            out.transform = Transform::from_look_towards(Vec3::X * 100. * i as f32, -Vec3::X); //most obj are stored facing X and not -Z, so we need to rotate them to face the camera
            out
        } ).collect();

        let mut inputs = Inputs::default_handler();

        safe_calls::set_clear_color(0., 0.5, 0.2);
        safe_calls::set_depth_test(true);
        safe_calls::set_cull_face(true);
        // safe_calls::set_draw_mode(Side::FrontAndBack, RenderMode::Lines);
        let mut mode = RenderMode::Full;

        let light = Vec3::new(0., 0., 100.);

        let uniform_light = program.uniform("light_pos");
        uniform_light.vec3(light);
        let mut light = Dot::new(light);
        light.bake(&program);
        
        for object in objects.iter_mut() {
            object.bake(&program);
        }
        // object.render_flags = 1;

        let size = ctx.window().inner_size();
        let uniform_proj = program.uniform("proj");
        uniform_proj.mat4(Matrix::projection(80f32.to_radians(), size.height as f32 / size.width as f32, 0.1, 10000.));
        
        let mut camera = Transform::from_look_at(Vec3::Z * 150., Vector::default());
        let uniform_camera = program.uniform("camera");
        uniform_camera.mat4(camera.as_view_matrix());

        let mut timer = std::time::Instant::now();
        
        let mut rotate = false;
        let mut speed_up = false;
        let mut fade_in = false;
        
        let mut focus = true;
        
        let mut fade = 0f32;
        let uniform_fade = program.uniform("fade");
        
        event_loop.run(move |event, _target, control_flow| {
            match event {
                Event::WindowEvent {
                    window_id, event,
                } if ctx.window().id() == window_id => {
                    if focus {
                        if let WindowEvent::KeyboardInput { input, .. } = event {
                            inputs.key_event(input);
                        }
                    }
                    if let WindowEvent::Resized(size) = event {
                        safe_calls::resize(size.width, size.height);
                        uniform_proj.mat4(Matrix::projection(80f32.to_radians(), size.height as f32 / size.width as f32, 0.1, 10000.));
                    }
                    if event == WindowEvent::CloseRequested {
                        *control_flow = ControlFlow::Exit
                    } else if let WindowEvent::Focused(focused) = event {
                        ctx.window().set_cursor_visible(!focused);
                        ctx.window().set_cursor_grab(focused).unwrap(); //should never produce an error since this project is to be tested on computers
                        focus = focused;
                    }
                }
                Event::DeviceEvent { event, .. } => {
                    match event {
                        DeviceEvent::MouseMotion { delta: (x, y) } => {
                            if focus {
                                let axis = Vec3::new(-y as f32, -x as f32, 0.).normalize();
                                let strength = (x * x + y * y).sqrt() as f32 / 10.;
                                // camera.rotate_local(axis, strength.to_radians());
                                camera.rotate_absolute(axis, strength.to_radians());
                                uniform_camera.mat4(camera.as_view_matrix());
                            }
                        }
                        _ => {}
                    }
                }
                Event::MainEventsCleared => {
                    let elapsed = timer.elapsed();
                    if elapsed.as_secs_f64() >= 1. / 60. {
                        timer = std::time::Instant::now();
                        if Inputs::apply_to_camera(&mut camera, &inputs, speed_up) {
                            uniform_camera.mat4(camera.as_view_matrix());
                        }
                        if inputs.status(Inputs::ToggleRotation).just_pressed() {
                            rotate = !rotate;
                        }
                        if inputs.status(Inputs::ToggleSpeedUp).just_pressed() {
                            speed_up = !speed_up;
                        }
                        if inputs.status(Inputs::ToggleFade).just_pressed() {
                            fade_in = !fade_in;
                        }
                        if inputs.status(Inputs::ToggleMode).just_pressed() {
                            mode = match mode {
                                RenderMode::Full => RenderMode::Lines,
                                RenderMode::Lines => RenderMode::Points,
                                RenderMode::Points => RenderMode::Full,
                            };
                            safe_calls::set_draw_mode(Side::FrontAndBack, mode);
                        }
                        safe_calls::clear_screen();
                        fade = (fade + if fade_in { 0.02 } else { -0.02 }).clamp(0., 1.);
                        uniform_fade.float(fade);
                        for object in objects.iter_mut() {
                            if rotate {
                                object.transform.rotate_local(Vec3::Y, 1f32.to_radians());
                            }
                            object.bind();
                            object.draw();
                        }
                        light.bind();
                        light.draw();
                        ctx.swap_buffers().unwrap();
                        inputs.tick();
                    }
                }
                _ => {}
            }
        });
    }
}
*/