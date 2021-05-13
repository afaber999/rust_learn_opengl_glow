use glow::*;
use glutin::event::{Event, VirtualKeyCode, WindowEvent, ElementState, MouseButton};
use glutin::event_loop::ControlFlow;
use std::rc::Rc;
use crate::{camera::{Camera, Directions}, shader::Shader};
extern crate nalgebra_glm as glm;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn gl_check_error_(gl : &glow::Context, file: &str, line: u32) -> u32 {
    unsafe {
        let mut error_code = gl.get_error();
        while error_code != glow::NO_ERROR {
            let error = match error_code {
                glow::INVALID_ENUM => "INVALID_ENUM",
                glow::INVALID_VALUE => "INVALID_VALUE",
                glow::INVALID_OPERATION => "INVALID_OPERATION",
                glow::STACK_OVERFLOW => "STACK_OVERFLOW",
                glow::STACK_UNDERFLOW => "STACK_UNDERFLOW",
                glow::OUT_OF_MEMORY => "OUT_OF_MEMORY",
                glow::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
                _ => "unknown GL error code"
            };
    
            println!("{} | {} ({})", error, file, line);
            error_code = gl.get_error();
        }
        error_code
    }
}

macro_rules! gl_check_error {
    ($expression:expr) => (
        gl_check_error_($expression, file!(), line!())
    )
}


pub fn main_7_1() {
    unsafe 
    {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("learn-opengl-glow => _1_debugging")
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
            "src/_7_in_practice/shaders/debugging.vs",
            "src/_7_in_practice/shaders/debugging.fs"
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

        // load textures
        // -------------
        // load and create a texture
        // -------------------------        
        let texture = Some( gl.create_texture().expect("Create a texture") );

        // bind texture, all upcoming GL_TEXTURE_2D operations now have effect on this texture object
        gl.bind_texture(glow::TEXTURE_2D, texture);

        // set the texture wrapping & repeat parameters
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);

        // set texture filtering parameters
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);

        // load image, create texture and generate mipmaps
        let img = image::open("resources/textures/wood.png").unwrap().flipv().into_rgb8();
        let (img_w, img_h) = img.dimensions();
        let raw_img = img.into_raw();
     
        // Give the image to OpenGL
        gl.tex_image_2d(glow::TEXTURE_2D,
                            0, 
                            glow::RGB as i32, 
                            img_w as i32, 
                            img_h as i32,
                            0, 
                            glow::RGB, 
                            glow::UNSIGNED_BYTE,
                            Some(&raw_img) );

        // enable to force error 1280, invalid enum
        // gl.tex_image_2d(glow::TEXTURE_3D,
        //                     0, 
        //                     glow::RGB as i32, 
        //                     img_w as i32, 
        //                     img_h as i32,
        //                     0, 
        //                     glow::RGB, 
        //                     glow::UNSIGNED_BYTE,
        //                     Some(&raw_img) );

        gl.generate_mipmap(glow::TEXTURE_2D);
        
        gl_check_error!(&gl);

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

                    // setup cube transformations
                    let aspect = SCR_WIDTH as f32/ SCR_HEIGHT as f32;
                    let projection = glm::perspective(aspect, camera.get_zoom().to_radians(), 0.1f32, 100.0f32);
                    let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(-1.0, 0.0, -1.0));
                    
                    shader.use_program();
                    gl.bind_vertex_array(Some(vao_cube));
                    gl.active_texture(glow::TEXTURE0);
                    gl.bind_texture(glow::TEXTURE_2D, texture);

                    shader.set_uniform_mat4("view", &camera.get_view_matrix());
                    shader.set_uniform_mat4("projection", &projection);                    
                    shader.set_uniform_mat4("model", &model);

                    gl.draw_arrays(glow::TRIANGLES, 0,  36);
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
                    gl.delete_buffer(vao_cube);
                },
                
                Event::NewEvents(_) => {}
                Event::DeviceEvent { device_id:_, event:_ } => {}
                Event::UserEvent(_) => {}
                Event::Suspended => {}
                Event::Resumed => {}
                Event::MainEventsCleared => {}
                Event::RedrawEventsCleared => {}
            }
        } );
    }
}

