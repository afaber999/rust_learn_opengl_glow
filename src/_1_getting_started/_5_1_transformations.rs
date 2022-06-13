use glow::*;
use glutin::event::{Event, VirtualKeyCode, WindowEvent};
use glutin::event_loop::ControlFlow;
use std::rc::Rc;
use crate::shader::Shader;
extern crate nalgebra_glm as glm;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main_1_5_1() {

    unsafe 
    {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("learn-opengl-glow => _5_1_transformations")
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
            "src/_1_getting_started/shaders/5.1.transform.vs",
            "src/_1_getting_started/shaders/5.1.transform.fs"
        );

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        // HINT: type annotation is crucial since default for float literals is f64
        let vertices:[f32;20] = [
            // positions     // texture coords
             0.5,  0.5, 0.0, 1.0, 1.0, // top right
             0.5, -0.5, 0.0, 1.0, 0.0, // bottom right
            -0.5, -0.5, 0.0, 0.0, 0.0, // bottom left
            -0.5,  0.5, 0.0, 0.0, 1.0  // top left 
        ];
        let indices = [
            0, 1, 3,  // first Triangle
            1, 2, 3   // second Triangle
        ];

        let vao = gl.create_vertex_array().expect("Create VAO");
        let vbo = gl.create_buffer().expect("Create VBO");
        let ebo = gl.create_buffer().expect("Create EBO");

        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        gl.bind_vertex_array( Some(vao) );
        
        gl.bind_buffer(glow::ARRAY_BUFFER, Some( vbo ));
        let u8_buffer = bytemuck::cast_slice(&vertices[..]);
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, u8_buffer, glow::STATIC_DRAW);

        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
        let u8_buffer = bytemuck::cast_slice(&indices[..]);
        gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, u8_buffer, glow::STATIC_DRAW);
       
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


        const DESIRED_FRAME_TIME :f32 = 0.02;
        let mut last_draw_time = std::time::Instant::now();
        let mut frame_time= 0.0f32;

        event_loop.run(move |event, _, control_flow| {
            
            let now =  std::time::Instant::now();
            let elapsed_time = now.duration_since(last_draw_time).as_secs_f32();

            if  elapsed_time > DESIRED_FRAME_TIME {
                frame_time += elapsed_time;
                window.window().request_redraw();
                last_draw_time = now;
            }

            match event {
                Event::RedrawRequested(_) => {
                    // DRAW HERE
                    gl.clear_color(0.2, 0.3, 0.3, 1.0);
                    gl.clear(glow::COLOR_BUFFER_BIT);                    

                    // bind textures on corresponding texture units
                    gl.active_texture(glow::TEXTURE0);
                    gl.bind_texture(glow::TEXTURE_2D, texture_1);
                    gl.active_texture(glow::TEXTURE1);
                    gl.bind_texture(glow::TEXTURE_2D, texture_2);


                    // create transformations
                    let transform = glm::translate(&glm::Mat4::identity(), &glm::vec3(0.5, -0.5, 0.0));
                    let transform = glm::rotate(&transform, frame_time, &glm::vec3(0.0, 0.0, 1.0));
                
                    // get matrix's uniform location and set matrix
                    shader.use_program();
                    shader.set_uniform_mat4("transform", &transform);

                    // render container
                    gl.bind_vertex_array(Some(vao));
                    gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT,0);

                    window.swap_buffers().unwrap();
                },

                Event::WindowEvent { ref event, .. } => match event {                    
                    WindowEvent::Resized(physical_size) => window.resize(*physical_size),
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { device_id:_, input, is_synthetic:_ } => {
                        match input.virtual_keycode {
                            Some(key) if key == VirtualKeyCode::Escape => {
                                *control_flow = glutin::event_loop::ControlFlow::Exit;
                            },
                            _ => (),
                        }
                    },
                    _=> {}
                },

                Event::LoopDestroyed => {
                    // CLEANUP  
                    gl.delete_buffer(vbo);
                    gl.delete_vertex_array(vao);
                    if let Some(id) = texture_1 { gl.delete_texture(id);}
                    if let Some(id) = texture_2 { gl.delete_texture(id);}
                },
                _ => {}
            }
        } );
    }
}
