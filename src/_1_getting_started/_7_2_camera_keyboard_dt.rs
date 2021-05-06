use glow::*;
use glutin::event::{Event, VirtualKeyCode, WindowEvent};
use glutin::event_loop::ControlFlow;
use std::rc::Rc;
use crate::shader::Shader;
extern crate nalgebra_glm as glm;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main_1_7_2() {

    unsafe 
    {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("learn-opengl-glow => _7_2_camera_keyboard_dt")
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
            "src/_1_getting_started/shaders/7.2.camera.vs",
            "src/_1_getting_started/shaders/7.2.camera.fs"
        );

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        // HINT: type annotation is crucial since default for float literals is f64
        let vertices:[f32; 180] = [
            // positions        texture coords
           -0.5, -0.5, -0.5,  0.0, 0.0,
            0.5, -0.5, -0.5,  1.0, 0.0,
            0.5,  0.5, -0.5,  1.0, 1.0,
            0.5,  0.5, -0.5,  1.0, 1.0,
           -0.5,  0.5, -0.5,  0.0, 1.0,
           -0.5, -0.5, -0.5,  0.0, 0.0,
   
           -0.5, -0.5,  0.5,  0.0, 0.0,
            0.5, -0.5,  0.5,  1.0, 0.0,
            0.5,  0.5,  0.5,  1.0, 1.0,
            0.5,  0.5,  0.5,  1.0, 1.0,
           -0.5,  0.5,  0.5,  0.0, 1.0,
           -0.5, -0.5,  0.5,  0.0, 0.0,
   
           -0.5,  0.5,  0.5,  1.0, 0.0,
           -0.5,  0.5, -0.5,  1.0, 1.0,
           -0.5, -0.5, -0.5,  0.0, 1.0,
           -0.5, -0.5, -0.5,  0.0, 1.0,
           -0.5, -0.5,  0.5,  0.0, 0.0,
           -0.5,  0.5,  0.5,  1.0, 0.0,
   
            0.5,  0.5,  0.5,  1.0, 0.0,
            0.5,  0.5, -0.5,  1.0, 1.0,
            0.5, -0.5, -0.5,  0.0, 1.0,
            0.5, -0.5, -0.5,  0.0, 1.0,
            0.5, -0.5,  0.5,  0.0, 0.0,
            0.5,  0.5,  0.5,  1.0, 0.0,
   
           -0.5, -0.5, -0.5,  0.0, 1.0,
            0.5, -0.5, -0.5,  1.0, 1.0,
            0.5, -0.5,  0.5,  1.0, 0.0,
            0.5, -0.5,  0.5,  1.0, 0.0,
           -0.5, -0.5,  0.5,  0.0, 0.0,
           -0.5, -0.5, -0.5,  0.0, 1.0,
   
           -0.5,  0.5, -0.5,  0.0, 1.0,
            0.5,  0.5, -0.5,  1.0, 1.0,
            0.5,  0.5,  0.5,  1.0, 0.0,
            0.5,  0.5,  0.5,  1.0, 0.0,
           -0.5,  0.5,  0.5,  0.0, 0.0,
           -0.5,  0.5, -0.5,  0.0, 1.0
        ];            

        // world space positions of our cubes
        let cube_positions:[_;10] = [
            glm::vec3( 0.0,  0.0,  0.0),
            glm::vec3( 2.0,  5.0, -15.0),
            glm::vec3(-1.5, -2.2, -2.5),
            glm::vec3(-3.8, -2.0, -12.0),
            glm::vec3( 2.4, -0.4, -3.5),
            glm::vec3(-1.7,  3.0, -7.5),
            glm::vec3( 1.3, -2.0, -2.5),
            glm::vec3( 1.5,  2.0, -2.5),
            glm::vec3( 1.5,  0.2, -1.5),
            glm::vec3(-1.3,  1.0, -1.5)
        ];

        let vao = gl.create_vertex_array().expect("Create VAO");
        let vbo = gl.create_buffer().expect("Create VBO");

        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        gl.bind_vertex_array( Some(vao) );
        
        gl.bind_buffer(glow::ARRAY_BUFFER, Some( vbo ));
        let u8_buffer = bytemuck::cast_slice(&vertices[..]);
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, u8_buffer, glow::STATIC_DRAW);
      
        // Vertex position attribute 0. No particular reason for 0, but must match the layout in the shader.
        gl.vertex_attrib_pointer_f32(
            0,              
            3,
            glow::FLOAT,
            false,
            std::mem::size_of::<f32>() as i32 * 5,
            0);

        gl.enable_vertex_attrib_array(0);
        
        // Texture coord attribute 2. No particular reason for 2, but must match the layout in the shader.
        gl.vertex_attrib_pointer_f32(
            1,              
            2,
            glow::FLOAT,
            false,
            std::mem::size_of::<f32>() as i32 * 5,
            std::mem::size_of::<f32>() as i32 * 3);

        gl.enable_vertex_attrib_array(1);

        // load and create a texture 1
        // -------------------------        
        let texture_1 = Some( gl.create_texture().expect("Create a texture") );

        // bind texture, all upcoming GL_TEXTURE_2D operations now have effect on this texture object
        gl.bind_texture(glow::TEXTURE_2D, texture_1);

        // set the texture wrapping & repeat parameters
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);

        // set texture filtering parameters
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);

        // load image, create texture and generate mipmaps
        let img = image::open("resources/textures/container.jpg").unwrap().flipv().into_rgb8();
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

        gl.generate_mipmap(glow::TEXTURE_2D);
        

        // load and create a texture 2
        // -------------------------        
        let texture_2 = Some( gl.create_texture().expect("Create a texture") );

        // bind texture, all upcoming GL_TEXTURE_2D operations now have effect on this texture object
        gl.bind_texture(glow::TEXTURE_2D, texture_2);

        // set the texture wrapping & repeat parameters
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);

        // set texture filtering parameters
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);

        // load image, create texture and generate mipmaps
        let img = image::open("resources/textures/awesomeface.png").unwrap().flipv().into_rgba8();
        let (img_w, img_h) = img.dimensions();
        let raw_img = img.into_raw();
     
        // Give the image to OpenGL
        gl.tex_image_2d(glow::TEXTURE_2D,
                            0, 
                            glow::RGB as i32, 
                            img_w as i32, 
                            img_h as i32,
                            0, 
                            glow::RGBA,  // include alpha channel
                            glow::UNSIGNED_BYTE,
                            Some(&raw_img) );

        gl.generate_mipmap(glow::TEXTURE_2D);
        
        // tell opengl for each sampler to which texture unit it belongs to (only has to be done once)
        // -------------------------------------------------------------------------------------------
        shader.use_program();

        // or set it via the texture class
        shader.set_uniform_i32("texture1", 0);
        shader.set_uniform_i32("texture2", 1);        

        let mut camera_pos  = glm::vec3(0.0, 0.0,  3.0);
        let camera_front= glm::vec3(0.0, 0.0, -1.0);
        let camera_up   = glm::vec3(0.0, 1.0,  0.0);


        const DESIRED_FRAME_TIME :f32 = 0.02;
        let mut last_draw_time = std::time::Instant::now();
        let mut _frame_time= 0.0f32;
        let camera_speed = 2.5f32 * DESIRED_FRAME_TIME;
        
        event_loop.run(move |event, _, control_flow| {
            
            let now =  std::time::Instant::now();
            let elapsed_time = now.duration_since(last_draw_time).as_secs_f32();

            if  elapsed_time > DESIRED_FRAME_TIME {
                _frame_time += elapsed_time;
                window.window().request_redraw();
                last_draw_time = now;
            }

            match event {
                Event::RedrawRequested(_) => {
                    // DRAW HERE
                    gl.clear_color(0.2, 0.3, 0.3, 1.0);

                    // enable depth test and clear the color and depth buffer
                    // disable gl.enable(glow::DEPTH_TEST) for excercise 6.2
                    gl.enable(glow::DEPTH_TEST);
                    gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

                    // bind textures on corresponding texture units
                    gl.active_texture(glow::TEXTURE0);
                    gl.bind_texture(glow::TEXTURE_2D, texture_1);
                    gl.active_texture(glow::TEXTURE1);
                    gl.bind_texture(glow::TEXTURE_2D, texture_2);


                    // create transformations
                    
                    let aspect = SCR_WIDTH as f32/ SCR_HEIGHT as f32;
                    let field_of_view = 45f32;

                    // setup view and projection matrix
                    let view = glm::look_at(
                        &camera_pos, 
                        &(camera_pos + camera_front), 
                        &camera_up);

                    let projection = glm::perspective(aspect, field_of_view.to_radians(), 0.1f32, 100.0f32);

                    // get matrix's uniform location and set matrix
                    shader.use_program();
                    shader.set_uniform_mat4("view", &view);
                    shader.set_uniform_mat4("projection", &projection);

                    // render container
                    gl.bind_vertex_array(Some(vao));

                    // draw a cube at all positions
                    for (idx, pos) in cube_positions.iter().enumerate() {

                        let mut model = glm::translate(&glm::Mat4::identity(), pos);
                        let angle = idx as f32 /3.0;
                        model = glm::rotate(&model,angle, &glm::vec3(1.0, 0.3, 0.5));
        
                        shader.set_uniform_mat4("model", &model);
                        gl.draw_arrays(glow::TRIANGLES, 0,  36);
                    }                    

                    window.swap_buffers().unwrap();
                },

                Event::WindowEvent { ref event, .. } => match event {                    
                    WindowEvent::Resized(physical_size) => window.resize(*physical_size),
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { device_id:_, input, is_synthetic:_ } => {
                        let up_down = camera_speed * camera_front;
                        let left_right = glm::normalize(&glm::cross(&camera_front, &camera_up)) * camera_speed;
                        match input.virtual_keycode {
                            Some(key) => {
                                match key {
                                    VirtualKeyCode::Escape => *control_flow = glutin::event_loop::ControlFlow::Exit,
                                    VirtualKeyCode::W => camera_pos += up_down,
                                    VirtualKeyCode::A => camera_pos -= left_right,
                                    VirtualKeyCode::S => camera_pos-= up_down,
                                    VirtualKeyCode::D => camera_pos += left_right,
                                    _ => (),
                                }
                            },
                            _ => (),
                        }
                    },
                    _=> {}
                },

                Event::LoopDestroyed => {
                    // CLEANUP  
                    gl.delete_buffer(vbo);
                    gl.delete_buffer(vao);
                    if let Some(id) = texture_1 { gl.delete_texture(id);}
                    if let Some(id) = texture_2 { gl.delete_texture(id);}
                },
                _ => {}
            }
        } );
    }
}
