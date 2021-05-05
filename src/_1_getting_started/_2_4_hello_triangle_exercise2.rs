use glow::*;
use glutin::event::{Event, VirtualKeyCode, WindowEvent};
use glutin::event_loop::ControlFlow;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main_1_2_4() {

    unsafe 
    {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("learn-opengl-glow => _2_4_hello_triangle_exercise2")
            .with_inner_size(glutin::dpi::LogicalSize::new(SCR_WIDTH, SCR_HEIGHT));
        let window = glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap()
            .make_current()
            .unwrap();
        let gl=glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
      
        let vx_source = r#"
            #version 330 core
            layout (location = 0) in vec3 aPos;
            void main() {
               gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
            }
        "#;

        let fg_source = r#"
            #version 330 core
            out vec4 FragColor;
            void main() {
               FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
            }
        "#;

        // build and compile our shader program
        // ------------------------------------
        // vertex shader        
        let program = gl.create_program().expect("Cannot create program");

        let vx_shader = gl
            .create_shader(glow::VERTEX_SHADER)
            .expect(&format!( "Cannot create shader: {}", glow::VERTEX_SHADER));
        
        gl.shader_source(vx_shader, vx_source);
        gl.compile_shader(vx_shader);

        // check for shader compile errors
        if !gl.get_shader_compile_status(vx_shader) {
            panic!( "{}", gl.get_shader_info_log(vx_shader));
        }
        gl.attach_shader(program, vx_shader);

        // fragment shader        
        let fg_shader = gl
            .create_shader(glow::FRAGMENT_SHADER)
            .expect(&format!( "Cannot create shader: {}", glow::FRAGMENT_SHADER));
        
        gl.shader_source(fg_shader, fg_source);
        gl.compile_shader(fg_shader);
        
        // check for shader compile errors
        if !gl.get_shader_compile_status(fg_shader) {
            panic!( "{}", gl.get_shader_info_log(fg_shader));
        }
        gl.attach_shader(program, fg_shader);

        // link shaders and check result
        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!( "{}", gl.get_program_info_log(program));
        }        

        gl.detach_shader(program, vx_shader);
        gl.delete_shader(vx_shader);

        gl.detach_shader(program, fg_shader);
        gl.delete_shader(fg_shader);

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        let first_triangle:[f32;9] = [
            -0.9, -0.5, 0.0,  // left
            -0.0, -0.5, 0.0,  // right
            -0.45, 0.5, 0.0,  // top
        ];
        let second_triangle:[f32;9] = [
            0.0, -0.5, 0.0,  // left
            0.9, -0.5, 0.0,  // right
            0.45, 0.5, 0.0   // top
        ];

        let vao1 = gl.create_vertex_array().expect("Create VAO1");
        let vao2 = gl.create_vertex_array().expect("Create VAO2");
        let vbo1 = gl.create_buffer().expect("Create VBO 1");
        let vbo2 = gl.create_buffer().expect("Create VBO 2");

        // first triangle setup
        // ---------------------        
        gl.bind_vertex_array( Some(vao1) );
        gl.bind_buffer(glow::ARRAY_BUFFER, Some( vbo1 ));
        let u8_buffer = bytemuck::cast_slice(&first_triangle[..]);
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, u8_buffer, glow::STATIC_DRAW);
       
        gl.vertex_attrib_pointer_f32(
            0,              // attribute 0. No particular reason for 0, but must match the layout in the shader.
            3,               // 3 elements
            glow::FLOAT,
            false,
            0,
            0 );

        gl.enable_vertex_attrib_array(0);
        
        // second triangle setup
        // ---------------------
        gl.bind_vertex_array( Some(vao2) );
        gl.bind_buffer(glow::ARRAY_BUFFER, Some( vbo2 ));
        let u8_buffer = bytemuck::cast_slice(&second_triangle[..]);
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, u8_buffer, glow::STATIC_DRAW);
       
        gl.vertex_attrib_pointer_f32(
            0,              // attribute 0. No particular reason for 0, but must match the layout in the shader.
            3,               // 3 elements
            glow::FLOAT,
            false,
            0,
            0 );

        gl.enable_vertex_attrib_array(0);

        // not really necessary as well, but beware of calls that could affect VAOs while this one is bound 
        // (like binding element buffer objects, or enabling/disabling vertex attributes)
        gl.bind_vertex_array(Some(0));                  

        // uncomment this call to draw in wireframe polygons.
        // gl.polygon_mode(glow::FRONT_AND_BACK, glow::LINE);

        event_loop.run(move |event, _, control_flow| {
            
            *control_flow = ControlFlow::WaitUntil( std::time::Instant::now() + std::time::Duration::from_millis(16));

            match event {
                Event::RedrawRequested(_) => {
                    // DRAW HERE
                    gl.clear_color(0.2, 0.3, 0.3, 1.0);
                    gl.clear(glow::COLOR_BUFFER_BIT);                    

                    gl.use_program(Some(program));

                    // draw first triangle
                    gl.bind_vertex_array(Some(vao1));
                    gl.draw_arrays(glow::TRIANGLES, 0,3);

                    // draw first triangle
                    gl.bind_vertex_array(Some(vao2));
                    gl.draw_arrays(glow::TRIANGLES, 0,3);

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
                    gl.delete_buffer(vbo1);
                    gl.delete_buffer(vbo2);
                    gl.delete_buffer(vao1);
                    gl.delete_buffer(vao2);
                    gl.delete_program(program);
                },
                _ => {}
            }
        } );
    }
}
