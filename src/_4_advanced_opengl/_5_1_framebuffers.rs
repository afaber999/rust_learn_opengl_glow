use glow::*;
use glutin::event::{Event, VirtualKeyCode, WindowEvent, ElementState, MouseButton};
use glutin::event_loop::ControlFlow;
use std::rc::Rc;
use crate::{camera::{Camera, Directions}, texture::Texture, shader::Shader};
extern crate nalgebra_glm as glm;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main_4_5_1() {

    unsafe 
    {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("learn-opengl-glow => _5_1_framebuffers")
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
            "src/_4_advanced_opengl/shaders/5.1.framebuffers.vs",
            "src/_4_advanced_opengl/shaders/5.1.framebuffers.fs"
        );

        let screen_shader = Shader::new_from_files(
            gl.clone(),
            "src/_4_advanced_opengl/shaders/5.1.framebuffers_screen.vs",
            "src/_4_advanced_opengl/shaders/5.1.framebuffers_screen.fs"
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
            // positions       // texture Coords
             5.0, -0.5,  5.0,  2.0, 0.0,
            -5.0, -0.5,  5.0,  0.0, 0.0,
            -5.0, -0.5, -5.0,  0.0, 2.0,

             5.0, -0.5,  5.0,  2.0, 0.0,
            -5.0, -0.5, -5.0,  0.0, 2.0,
             5.0, -0.5, -5.0,  2.0, 2.0
        ];

        let quad_vertices: [f32; 24] = [
        // vertex attributes for a quad that fills the entire screen in Normalized Device Coordinates.
        // positions   // texCoords
        -1.0,  1.0,  0.0, 1.0,
        -1.0, -1.0,  0.0, 0.0,
         1.0, -1.0,  1.0, 0.0,

        -1.0,  1.0,  0.0, 1.0,
         1.0, -1.0,  1.0, 0.0,
         1.0,  1.0,  1.0, 1.0
        ];        

        // first configure the cube
        // -----------------------------------
        let cube_vao = gl.create_vertex_array().expect("Create VAO");
        let cube_vbo = gl.create_buffer().expect("Create VBO");

        gl.bind_vertex_array( Some(cube_vao) );
        
        gl.bind_buffer(glow::ARRAY_BUFFER, Some( cube_vbo ));
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
        let plane_vao = gl.create_vertex_array().expect("Create VAO");
        let plane_vbo = gl.create_buffer().expect("Create VBO");

        gl.bind_vertex_array( Some(plane_vao) );
        
        gl.bind_buffer(glow::ARRAY_BUFFER, Some( plane_vbo ));
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

        // screen VAO
        let screen_vao = gl.create_vertex_array().expect("Create VAO");
        let screen_vbo = gl.create_buffer().expect("Create VBO");

        gl.bind_vertex_array( Some(screen_vao) );
        
        gl.bind_buffer(glow::ARRAY_BUFFER, Some( screen_vbo ));
        let u8_buffer = bytemuck::cast_slice(&quad_vertices[..]);
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, u8_buffer, glow::STATIC_DRAW);
      
        // Vertex position attribute 0.
        gl.vertex_attrib_pointer_f32(
            0,              
            2,
            glow::FLOAT,
            false,
            std::mem::size_of::<f32>() as i32 * 4,
            0);

        gl.enable_vertex_attrib_array(0);
        
        // Texture attribute 1.
        gl.vertex_attrib_pointer_f32(
            1,              
            2,
            glow::FLOAT,
            false,
            std::mem::size_of::<f32>() as i32 * 4,
            std::mem::size_of::<f32>() as i32 * 2);

        gl.enable_vertex_attrib_array(1);
        gl.bind_vertex_array(None);


        // load textures
        // -------------
        let cube_texture = Texture::new(gl.clone(),"resources/textures/container.jpg",true);
        let floor_texture = Texture::new(gl.clone(),"resources/textures/metal.png",true);

        // shader configuration
        // --------------------
        shader.use_program();
        shader.set_uniform_i32("texture1", 0);

        screen_shader.use_program();
        screen_shader.set_uniform_i32("screenTexture", 0);

        // framebuffer configuration
        // -------------------------
        let frame_buffer = gl.create_framebuffer().expect("Create frame buffer");
        gl.bind_framebuffer(glow::FRAMEBUFFER, Some(frame_buffer));

        // create a color attachment texture
        let color_buffer_texture = gl.create_texture().expect("Create texture");
        gl.bind_texture(glow::TEXTURE_2D, Some(color_buffer_texture));
        gl.tex_image_2d(
            glow::TEXTURE_2D, 
            0, 
            glow::RGB as i32,
            SCR_WIDTH as i32,
            SCR_HEIGHT as i32, 
            0,
            glow::RGB,
            glow::UNSIGNED_BYTE,
            None);
        
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);

        gl.framebuffer_texture_2d(
            glow::FRAMEBUFFER, 
            glow::COLOR_ATTACHMENT0, 
            glow::TEXTURE_2D, 
            Some( color_buffer_texture),
            0);

        // create a renderbuffer object for depth and stencil attachment (we won't be sampling these)
        let rbo = gl.create_renderbuffer().expect("Create rbo");
        gl.bind_renderbuffer(glow::RENDERBUFFER, Some(rbo));

        // use a single renderbuffer object for both a depth AND stencil buffer.
        gl.renderbuffer_storage(
            glow::RENDERBUFFER,
            glow::DEPTH24_STENCIL8,
            SCR_WIDTH as i32,
            SCR_HEIGHT as i32);

        // now actually attach it
        gl.framebuffer_renderbuffer(
            glow::FRAMEBUFFER, 
            glow::DEPTH_STENCIL_ATTACHMENT, 
            glow::RENDERBUFFER, 
            Some(rbo)); 

        if gl.check_framebuffer_status(glow::FRAMEBUFFER) != glow::FRAMEBUFFER_COMPLETE {
            panic!("ERROR::FRAMEBUFFER:: Framebuffer is not complete!");
        }
        gl.bind_framebuffer(glow::FRAMEBUFFER, None);

        // Enable this line to show that the final image is drawn from two 
        // triangles, using the texture from the off-screen buffer
        //gl.polygon_mode(glow::FRONT_AND_BACK, glow::LINE);

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

                    // bind to framebuffer and draw scene as we normally would to color texture 
                    gl.bind_framebuffer(glow::FRAMEBUFFER, Some(frame_buffer));

                    // enable depth testing (is disabled for rendering screen-space quad)
                    gl.enable(glow::DEPTH_TEST); 


                    gl.clear_color(0.1, 0.1, 0.1, 1.0);
                    gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

                    // setup cube transformations
                    let aspect = SCR_WIDTH as f32/ SCR_HEIGHT as f32;
                    let projection = glm::perspective(aspect, camera.get_zoom().to_radians(), 0.1f32, 100.0f32);
                    let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(-1.0, 0.0, -1.0));
                    
                    shader.use_program();
                    gl.bind_vertex_array(Some(cube_vao));
                    gl.active_texture(glow::TEXTURE0);
                    cube_texture.bind();

                    shader.set_uniform_mat4("view", &camera.get_view_matrix());
                    shader.set_uniform_mat4("projection", &projection);                    
                    shader.set_uniform_mat4("model", &model);

                    gl.draw_arrays(glow::TRIANGLES, 0,  36);

                    // draw plane
                    gl.bind_vertex_array(Some(plane_vao));
                    let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(-1.0, 0.0, -1.0));
                    shader.set_uniform_mat4("model", &model);
                    floor_texture.bind();
                    gl.draw_arrays(glow::TRIANGLES, 0,  6);

                    gl.bind_vertex_array(None);

                    // now bind back to default framebuffer and draw a quad plane with the attached framebuffer color texture
                    gl.bind_framebuffer(glow::FRAMEBUFFER, None);
                    // disable depth test so screen-space quad isn't discarded due to depth test.
                    gl.disable(glow::DEPTH_TEST);
                    
                    // set clear color to white (not really necessary actually, since we won't be able to see behind the quad anyways)
                    gl.clear_color(1.0, 1.0, 1.0, 1.0);
                    gl.clear(glow::COLOR_BUFFER_BIT);

                    screen_shader.use_program();
                    gl.bind_vertex_array( Some(screen_vao));
                    gl.bind_texture(glow::TEXTURE_2D, Some(color_buffer_texture));
                    // use the color attachment texture as the texture of the quad plane
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
                    gl.delete_buffer(cube_vbo);
                    gl.delete_buffer(cube_vao);
                    gl.delete_buffer(plane_vbo);
                    gl.delete_buffer(plane_vao);
                    gl.delete_buffer(screen_vbo);
                    gl.delete_buffer(screen_vao);
                },
                _ => {}
            }
        } );
    }
}

