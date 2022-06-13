use glow::*;
use glutin::event::{Event, VirtualKeyCode, WindowEvent, ElementState, MouseButton};
use glutin::event_loop::ControlFlow;
use std::rc::Rc;
use crate::{camera::{Camera, Directions}, texture::Texture, shader::Shader};
extern crate nalgebra_glm as glm;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main_4_1_1() {

    unsafe 
    {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("learn-opengl-glow => _1_1_depth_testing")
            .with_inner_size(glutin::dpi::LogicalSize::new(SCR_WIDTH, SCR_HEIGHT));
        let window = glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap()
            .make_current()
            .unwrap();
        let gl=Rc::new(glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _));
      

        let shader = Shader::new_from_files(
            gl.clone(),
            "src/_4_advanced_opengl/shaders/1.1.depth_testing.vs",
            "src/_4_advanced_opengl/shaders/1.1.depth_testing.fs"
        );

        // set up vertex data (and buffer(s)) and configure vertex attributes
        let vertices: [f32; 180] = [
            // positions       // texture coords
            -0.5, -0.5, -0.5,  0.0,  0.0,
             0.5, -0.5, -0.5,  1.0,  0.0,
             0.5,  0.5, -0.5,  1.0,  1.0,
             0.5,  0.5, -0.5,  1.0,  1.0,
            -0.5,  0.5, -0.5,  0.0,  1.0,
            -0.5, -0.5, -0.5,  0.0,  0.0,

            -0.5, -0.5,  0.5,  0.0,  0.0,
             0.5, -0.5,  0.5,  1.0,  0.0,
             0.5,  0.5,  0.5,  1.0,  1.0,
             0.5,  0.5,  0.5,  1.0,  1.0,
            -0.5,  0.5,  0.5,  0.0,  1.0,
            -0.5, -0.5,  0.5,  0.0,  0.0,

            -0.5,  0.5,  0.5,  1.0,  0.0,
            -0.5,  0.5, -0.5,  1.0,  1.0,
            -0.5, -0.5, -0.5,  0.0,  1.0,
            -0.5, -0.5, -0.5,  0.0,  1.0,
            -0.5, -0.5,  0.5,  0.0,  0.0,
            -0.5,  0.5,  0.5,  1.0,  0.0,

             0.5,  0.5,  0.5,  1.0,  0.0,
             0.5,  0.5, -0.5,  1.0,  1.0,
             0.5, -0.5, -0.5,  0.0,  1.0,
             0.5, -0.5, -0.5,  0.0,  1.0,
             0.5, -0.5,  0.5,  0.0,  0.0,
             0.5,  0.5,  0.5,  1.0,  0.0,

            -0.5, -0.5, -0.5,  0.0,  1.0,
             0.5, -0.5, -0.5,  1.0,  1.0,
             0.5, -0.5,  0.5,  1.0,  0.0,
             0.5, -0.5,  0.5,  1.0,  0.0,
            -0.5, -0.5,  0.5,  0.0,  0.0,
            -0.5, -0.5, -0.5,  0.0,  1.0,

            -0.5,  0.5, -0.5,  0.0,  1.0,
             0.5,  0.5, -0.5,  1.0,  1.0,
             0.5,  0.5,  0.5,  1.0,  0.0,
             0.5,  0.5,  0.5,  1.0,  0.0,
            -0.5,  0.5,  0.5,  0.0,  0.0,
            -0.5,  0.5, -0.5,  0.0,  1.0
        ];

        let plane_vertices: [f32; 30] = [
            // positions       // texture Coords (note we set these higher than 1 (together with GL_REPEAT as texture wrapping mode). this will cause the floor texture to repeat)
             5.0, -0.5,  5.0,  2.0, 0.0,
            -5.0, -0.5,  5.0,  0.0, 0.0,
            -5.0, -0.5, -5.0,  0.0, 2.0,

             5.0, -0.5,  5.0,  2.0, 0.0,
            -5.0, -0.5, -5.0,  0.0, 2.0,
             5.0, -0.5, -5.0,  2.0, 2.0
        ];

        // first configure the cube
        // -----------------------------------
        let vao_cube = gl.create_vertex_array().expect("Create VAO");
        let vbo = gl.create_buffer().expect("Create VBO");

        gl.bind_vertex_array( Some(vao_cube) );
        
        gl.bind_buffer(glow::ARRAY_BUFFER, Some( vbo ));
        let u8_buffer = bytemuck::cast_slice(&vertices[..]);
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, u8_buffer, glow::STATIC_DRAW);
      
        // Vertex position attribute 0.
        gl.vertex_attrib_pointer_f32(
            0,              
            3,
            glow::FLOAT,
            false,
            std::mem::size_of::<f32>() as i32 * 5,
            0);

        gl.enable_vertex_attrib_array(0);
        
        // Texture attribute 1.
        gl.vertex_attrib_pointer_f32(
            1,              
            2,
            glow::FLOAT,
            false,
            std::mem::size_of::<f32>() as i32 * 5,
            std::mem::size_of::<f32>() as i32 * 3);

        gl.enable_vertex_attrib_array(1);
        gl.bind_vertex_array(None);

        // plane VAO
        let vao_plane = gl.create_vertex_array().expect("Create VAO");
        let vbo_plane = gl.create_buffer().expect("Create VBO");

        gl.bind_vertex_array( Some(vao_plane) );
        
        gl.bind_buffer(glow::ARRAY_BUFFER, Some( vbo_plane ));
        let u8_buffer = bytemuck::cast_slice(&plane_vertices[..]);
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, u8_buffer, glow::STATIC_DRAW);
      
        // Vertex position attribute 0.
        gl.vertex_attrib_pointer_f32(
            0,              
            3,
            glow::FLOAT,
            false,
            std::mem::size_of::<f32>() as i32 * 5,
            0);

        gl.enable_vertex_attrib_array(0);
        
        // Texture attribute 1.
        gl.vertex_attrib_pointer_f32(
            1,              
            2,
            glow::FLOAT,
            false,
            std::mem::size_of::<f32>() as i32 * 5,
            std::mem::size_of::<f32>() as i32 * 3);

        gl.enable_vertex_attrib_array(1);
        gl.bind_vertex_array(None);

        // load textures
        // -------------
        let cube_texture = Texture::new(gl.clone(),"resources/textures/marble.jpg",true);
        let floor_texture = Texture::new(gl.clone(),"resources/textures/metal.png",true);

        // shader configuration
        // --------------------
        shader.use_program();
        shader.set_uniform_i32("texture1", 0);

        let mut camera = Camera::new(glm::vec3( 0.0, 0.0, 5.0));

        let mut is_dragging = false;
        let mut last_x = SCR_WIDTH as f32 / 2.0;
        let mut last_y = SCR_HEIGHT as f32 / 2.0;

        const DESIRED_FRAME_TIME :f32 = 0.02;
        let mut last_draw_time = std::time::Instant::now();
        
        event_loop.run(move |event, _, control_flow| {
            
            let now =  std::time::Instant::now();
            let elapsed_time = now.duration_since(last_draw_time).as_secs_f32();

            if  elapsed_time > DESIRED_FRAME_TIME {
                window.window().request_redraw();
                last_draw_time = now;
            }

            match event {
                Event::RedrawRequested(_) => {
                    // DRAW HERE
                    gl.clear_color(0.1, 0.1, 0.1, 1.0);
                    gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

                    // switch between depth func LESS and ALWAYS.
                    // Notice the difference, when set to ALWAYS, 
                    // the plane is fully drawn in front of cube
                    gl.enable(glow::DEPTH_TEST);
                    //gl.depth_func(glow::LESS);
                    gl.depth_func(glow::ALWAYS);

                    // setup cube transformations
                    let aspect = SCR_WIDTH as f32/ SCR_HEIGHT as f32;
                    let projection = glm::perspective(aspect, camera.get_zoom().to_radians(), 0.1f32, 100.0f32);
                    let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(-1.0, 0.0, -1.0));
                    
                    shader.use_program();
                    gl.bind_vertex_array(Some(vao_cube));
                    gl.active_texture(glow::TEXTURE0);
                    cube_texture.bind();

                    shader.set_uniform_mat4("view", &camera.get_view_matrix());
                    shader.set_uniform_mat4("projection", &projection);                    
                    shader.set_uniform_mat4("model", &model);

                    gl.draw_arrays(glow::TRIANGLES, 0,  36);

                    // draw plane
                    gl.bind_vertex_array(Some(vao_plane));
                    let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(-1.0, 0.0, -1.0));
                    shader.set_uniform_mat4("model", &model);
                    floor_texture.bind();
                    gl.draw_arrays(glow::TRIANGLES, 0,  6);

                    gl.bind_vertex_array(None);
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
                    gl.delete_buffer(vbo);
                    gl.delete_vertex_array(vao_cube);
                    gl.delete_buffer(vbo_plane);
                    gl.delete_vertex_array(vao_plane);
                },
                _ => {}
            }
        } );
    }
}

