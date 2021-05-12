use glow::*;
use glutin::event::{Event, VirtualKeyCode, WindowEvent, ElementState, MouseButton};
use glutin::event_loop::ControlFlow;
use std::rc::Rc;
use crate::{model::Model, camera::{Camera, Directions}, shader::Shader};
extern crate nalgebra_glm as glm;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;


pub fn main_3_1() {
    unsafe 
    {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("learn-opengl-glow => _1_model_loading")
            .with_inner_size(glutin::dpi::LogicalSize::new(SCR_WIDTH, SCR_HEIGHT));
        let window = glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap()
            .make_current()
            .unwrap();
        let gl=Rc::new( glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _));
        
        //let our_model = Model::new(gl.clone(), "resources/objects/backpack/backpack.obj");
        let our_model = Model::new(gl.clone(), "resources/objects/nanosuit/nanosuit.obj");
        //let our_model = Model::new(gl.clone(), "resources/objects/rock/rock.obj");
        //let our_model = Model::new(gl.clone(), "resources/objects/cube/cube.obj");
        
        let shader = Shader::new_from_files(
            gl.clone(),
            "src/_3_model_loading/shaders/1.model_loading.vs",
            "src/_3_model_loading/shaders/1.model_loading.fs"
        );

        let mut camera = Camera::new(glm::vec3( 2.0, 2.0, 10.0));

        let mut is_dragging = false;
        let mut last_x = SCR_WIDTH as f32 / 2.0;
        let mut last_y = SCR_HEIGHT as f32 / 2.0;

        const DESIRED_FRAME_TIME :f32 = 0.02;
        let mut last_draw_time = std::time::Instant::now();
        let mut _frame_time= 0.0f32;
        
        event_loop.run(move |event, _, control_flow| {

            let now =  std::time::Instant::now();
            let elapsed_time = now.duration_since(last_draw_time).as_secs_f32();

            if  elapsed_time > DESIRED_FRAME_TIME {
                _frame_time += elapsed_time;
                window.window().request_redraw();
                last_draw_time = now;
            }

            // wireframe
            //gl.polygon_mode(glow::FRONT_AND_BACK, glow::LINE);

            match event {
                Event::RedrawRequested(_) => {
                    // DRAW HERE
                    gl.clear_color(0.2, 0.3, 0.3, 1.0);
                    gl.enable(glow::DEPTH_TEST);
                    gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

                    let aspect = SCR_WIDTH as f32/ SCR_HEIGHT as f32;
                    let projection = glm::perspective(aspect, camera.get_zoom().to_radians(), 0.1f32, 100.0f32);
                    let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(0.0, -1.75, 0.0));

                    shader.use_program();
                    shader.set_uniform_mat4("view", &camera.get_view_matrix());
                    shader.set_uniform_mat4("projection", &projection);
                    shader.set_uniform_mat4("model", &model);

                    our_model.draw(&shader);
                    window.swap_buffers().unwrap();
                },

                Event::WindowEvent { ref event, .. } => match event {                    
                    WindowEvent::Resized(physical_size) => window.resize(*physical_size),
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { device_id:_, input, is_synthetic:_ } => {
                        match input.virtual_keycode {
                            Some(key) => {
                                match key {
                                    VirtualKeyCode::Escape => *control_flow = glutin::event_loop::ControlFlow::Exit,
                                    VirtualKeyCode::W    => camera.key_interact(Directions::Forward),
                                    VirtualKeyCode::A    => camera.key_interact(Directions::Left),
                                    VirtualKeyCode::S    => camera.key_interact(Directions::Backward),
                                    VirtualKeyCode::D    => camera.key_interact(Directions::Right),
                                    VirtualKeyCode::Up   => camera.key_interact(Directions::Up),
                                    VirtualKeyCode::Down => camera.key_interact(Directions::Down),
                                    VirtualKeyCode::Left => camera.key_interact(Directions::Left),
                                    VirtualKeyCode::Right=> camera.key_interact(Directions::Right),
                                    _ => (),
                                }
                            },
                            _ => (),
                        }
                    },
                    WindowEvent::CursorMoved { device_id:_, position, .. } => {
                        //println!("Move to {:?}", position);
                        let new_x = position.x as f32;
                        let new_y = position.y as f32;

                        if is_dragging {
                            camera.mouse_interact(new_x - last_x, new_y - last_y);                            
                        }
                        last_x = new_x;
                        last_y = new_y;
                    },

                    WindowEvent::MouseInput { device_id:_, state, button, .. } => {
                        if state == &ElementState::Pressed && button == &MouseButton::Left {
                            is_dragging = true;
                        } else {
                            is_dragging = false;
                        }
                    },

                    WindowEvent::MouseWheel { device_id:_, delta, phase :_, .. } => {
                        match delta {
                            glutin::event::MouseScrollDelta::LineDelta(_x,y) => {
                                camera.scroll_wheel_interact(*y);
                            },
                            _ => (),
                        }
                    },
                    _=> {}
                },

                Event::LoopDestroyed => {
                    // CLEANUP  
                },
                _ => {}
            }
        } );
    }
}
