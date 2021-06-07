use glow::*;
use glutin::event::{Event, VirtualKeyCode, WindowEvent, ElementState, MouseButton};
use glutin::event_loop::ControlFlow;
use std::rc::Rc;
use crate::{camera::{Camera, Directions}, texture::Texture, shader::Shader};
extern crate nalgebra_glm as glm;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main_2_5_2() {

    unsafe 
    {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("learn-opengl-glow => _5_2_light_casters_point")
            .with_inner_size(glutin::dpi::LogicalSize::new(SCR_WIDTH, SCR_HEIGHT));
        let window = glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap()
            .make_current()
            .unwrap();
        let gl=Rc::new(glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _));
      

        let shader_cube = Shader::new_from_files(
            gl.clone(),
            "src/_2_lighting/shaders/5.2.light_casters.vs",
            "src/_2_lighting/shaders/5.2.light_casters.fs"
        );

        let shader_light = Shader::new_from_files(
            gl.clone(),
            "src/_2_lighting/shaders/5.2.lamp.vs",
            "src/_2_lighting/shaders/5.2.lamp.fs"
        );


        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        // HINT: type annotation is crucial since default for float literals is f64
        let vertices: [f32; 288] = [
            // positions       // normals        // texture coords
            -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,
             0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  0.0,
             0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
             0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
            -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  1.0,
            -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,

            -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,
             0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  0.0,
             0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
             0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
            -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  1.0,
            -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,

            -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,
            -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,  1.0,  1.0,
            -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
            -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
            -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,  0.0,  0.0,
            -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,

             0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,
             0.5,  0.5, -0.5,  1.0,  0.0,  0.0,  1.0,  1.0,
             0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
             0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
             0.5, -0.5,  0.5,  1.0,  0.0,  0.0,  0.0,  0.0,
             0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,

            -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,
             0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  1.0,  1.0,
             0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
             0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
            -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  0.0,  0.0,
            -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,

            -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0,
             0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  1.0,  1.0,
             0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
             0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
            -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  0.0,  0.0,
            -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0
        ];

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

        // first configure the cube
        // -----------------------------------
        let vao_cube = gl.create_vertex_array().expect("Create VAO");
        let vbo = gl.create_buffer().expect("Create VBO");

        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
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
            std::mem::size_of::<f32>() as i32 * 8,
            0);

        gl.enable_vertex_attrib_array(0);
        
        // Normal attribute 1.
        gl.vertex_attrib_pointer_f32(
            1,              
            3,
            glow::FLOAT,
            false,
            std::mem::size_of::<f32>() as i32 * 8,
            std::mem::size_of::<f32>() as i32 * 3);

        gl.enable_vertex_attrib_array(1);

        // Texture attribute 1.
        gl.vertex_attrib_pointer_f32(
            2,              
            2,
            glow::FLOAT,
            false,
            std::mem::size_of::<f32>() as i32 * 8,
            std::mem::size_of::<f32>() as i32 * 6);

        gl.enable_vertex_attrib_array(2);

        // second configure the light source
        // -----------------------------------
        let vao_light = gl.create_vertex_array().expect("Create VAO");
        gl.bind_vertex_array( Some(vao_light) );
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));    
        
        // Vertex position attribute 0.
        gl.vertex_attrib_pointer_f32(
            0,              
            3,
            glow::FLOAT,
            false,
            std::mem::size_of::<f32>() as i32 * 8,
            0);

        gl.enable_vertex_attrib_array(0);

        let diffuse_map = Texture::new(gl.clone(),"resources/textures/container2.png",true);
        let specular_map = Texture::new(gl.clone(),"resources/textures/container2_specular.png",true);

        // shader configuration
        // --------------------
        shader_cube.use_program();

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

                    // enable depth test and clear the color and depth buffer
                    gl.enable(glow::DEPTH_TEST);
                    gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

                    // setup cube transformations
                    let aspect = SCR_WIDTH as f32/ SCR_HEIGHT as f32;
                    let projection = glm::perspective(aspect, camera.get_zoom().to_radians(), 0.1f32, 100.0f32);
                    
                    shader_cube.use_program();

                    // light properties
                    let light_pos = glm::vec3(1.2, 1.0, 2.0);

                    shader_cube.set_uniform_vec3("light.position",&light_pos);
                    shader_cube.set_uniform_3_f32("light.ambient", 0.2,0.2,0.2);
                    shader_cube.set_uniform_3_f32("light.diffuse", 0.5,0.5,0.5);
                    shader_cube.set_uniform_3_f32("light.specular", 1.0, 1.0, 1.0);
        
                    shader_cube.set_uniform_f32("light.constant", 1.0);
                    shader_cube.set_uniform_f32("light.linear", 0.09);
                    shader_cube.set_uniform_f32("light.quadratic", 0.032);
                    
                    // material properties
                    shader_cube.set_uniform_i32("material.diffuse", 0); // point to diffuse map index!
                    shader_cube.set_uniform_i32("material.specular", 1); // point to specular map index!
                    shader_cube.set_uniform_f32("material.shininess", 32.0);

                    shader_cube.set_uniform_vec3("viewPos", &camera.get_position()); 
                    shader_cube.set_uniform_mat4("view", &camera.get_view_matrix());
                    shader_cube.set_uniform_mat4("projection", &projection);
                    
                    // setup texture
                    gl.active_texture(glow::TEXTURE0);
                    diffuse_map.bind();
                    gl.active_texture(glow::TEXTURE1);
                    specular_map.bind();

                    // render cube
                    gl.bind_vertex_array(Some(vao_cube));

                    for (idx, pos) in cube_positions.iter().enumerate() {

                        let mut model = glm::translate(&glm::Mat4::identity(), pos);
                        let angle = idx as f32 /3.0;
                        model = glm::rotate(&model,angle, &glm::vec3(1.0, 0.3, 0.5));
        
                        shader_cube.set_uniform_mat4("model", &model);
                        gl.draw_arrays(glow::TRIANGLES, 0,  36);
                    }                    

                    // setup light source transformations (only the object model changes)
                    let model =  glm::scale( &glm::translate(
                        &glm::Mat4::identity(), 
                        &light_pos), &glm::vec3(0.2,0.2,0.2) );
                    
                    shader_light.use_program();
                    shader_light.set_uniform_mat4("view", &camera.get_view_matrix());
                    shader_light.set_uniform_mat4("projection", &projection);
                    shader_light.set_uniform_mat4("model", &model);

                    // render light source
                    gl.bind_vertex_array(Some(vao_light));
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
                    gl.delete_buffer(vao_light);
                },
                _ => {}
            }
        } );
    }
}
