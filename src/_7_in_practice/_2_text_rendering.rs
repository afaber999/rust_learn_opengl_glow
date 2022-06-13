
use glow::*;
use std::{path::Path, rc::Rc};
use glutin::event::{Event, VirtualKeyCode, WindowEvent};
use glutin::event_loop::ControlFlow;
use crate::{shader::Shader, texture::Texture};
extern crate nalgebra_glm as glm;
use ab_glyph::{point, Point, Font, FontVec, Glyph, PxScale, ScaleFont};
use image::{DynamicImage, Rgba};

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;


const TEXT: &str = "This is ab_glyph rendered into a png!";

pub fn layout_paragraph<F, SF>(
    font: SF,
    position: Point,
    max_width: f32,
    text: &str,
    target: &mut Vec<Glyph>,
) where
    F: Font,
    SF: ScaleFont<F>,
{
    let v_advance = font.height() + font.line_gap();
    let mut caret = position + point(0.0, font.ascent());
    let mut last_glyph: Option<Glyph> = None;
    for c in text.chars() {
        if c.is_control() {
            if c == '\n' {
                caret = point(position.x, caret.y + v_advance);
                last_glyph = None;
            }
            continue;
        }
        let mut glyph = font.scaled_glyph(c);
        if let Some(previous) = last_glyph.take() {
            caret.x += font.kern(previous.id, glyph.id);
        }
        glyph.position = caret;

        last_glyph = Some(glyph.clone());
        caret.x += font.h_advance(glyph.id);

        if !c.is_whitespace() && caret.x > position.x + max_width {
            caret = point(position.x, caret.y + v_advance);
            glyph.position = caret;
            last_glyph = None;
        }

        target.push(glyph);
    }
}

fn draw_image<F: Font>(font: F) {
    // The font size to use
    let scale = PxScale::from(45.0);

    let scaled_font = font.as_scaled(scale);

    let mut glyphs = Vec::new();
    layout_paragraph(scaled_font, point(20.0, 20.0), 9999.0, TEXT, &mut glyphs);

    // Use a dark red colour
    let colour = (150, 0, 0);

    // work out the layout size
    let glyphs_height = scaled_font.height().ceil() as u32;
    let glyphs_width = {
        let min_x = glyphs.first().unwrap().position.x;
        let last_glyph = glyphs.last().unwrap();
        let max_x = last_glyph.position.x + scaled_font.h_advance(last_glyph.id);
        (max_x - min_x).ceil() as u32
    };

    // Create a new rgba image with some padding
    let mut image = DynamicImage::new_rgba8(glyphs_width + 40, glyphs_height + 40).to_rgba8();

    // Loop through the glyphs in the text, positing each one on a line
    for glyph in glyphs {
        if let Some(outlined) = scaled_font.outline_glyph(glyph) {
            let bounds = outlined.px_bounds();
            // Draw the glyph into the image per-pixel by using the draw closure
            outlined.draw(|x, y, v| {
                // Offset the position by the glyph bounding box
                let px = image.get_pixel_mut(x + bounds.min.x as u32, y + bounds.min.y as u32);
                // Turn the coverage into an alpha value (blended with any previous)
                *px = Rgba([
                    colour.0,
                    colour.1,
                    colour.2,
                    px.0[3].saturating_add((v * 255.0) as u8),
                ]);
            });
        }
    }

    // Save the image to a png file
    //image.save("image_example.png").unwrap();
    //println!("Generated: image_example.png");
}


pub fn main_7_2() {

    let font_path = Path::new( "resources/fonts/Antonio-Bold.ttf");

    let data = std::fs::read(&font_path).unwrap();
    let font = FontVec::try_from_vec(data).unwrap_or_else(|_| {
        panic!(
            "error constructing a Font from data at {:?}",
            font_path
        );
    });
    if let Some(name) = font_path.file_name().and_then(|n| n.to_str()) {
        eprintln!("Using font: {}", name);
    }
    draw_image(font);
    
    
    unsafe 
    {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("learn-opengl-glow => _7_2_text_rendering")
            .with_inner_size(glutin::dpi::LogicalSize::new(SCR_WIDTH, SCR_HEIGHT));
        let window = glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap()
            .make_current()
            .unwrap();
        let gl= Rc::new( glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _));
      
        // OpenGL state
        // ------------
        gl.enable(glow::CULL_FACE);
        gl.enable(glow::BLEND);
        gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);

        let shader = Shader::new_from_files(
            gl.clone(), 
            "src/_7_in_practice/shaders/text.vs",
            "src/_7_in_practice/shaders/text.fs");

        let projection = glm::ortho(0.0, SCR_WIDTH as f32, 0.0,SCR_HEIGHT as f32, -1.0, 1.0);

        shader.use_program();
        shader.set_uniform_mat4("projection", &projection);

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        // HINT: type annotation is crucial since default for float literals is f64
        let x_pos = 50.0f32;
        let y_pos = 100.0f32;
        let w = 145.0f32;
        let h = 125.0f32;

        let vertices: [f32; 24] = [
            x_pos,     y_pos + h,   0.0, 0.0,            
            x_pos,     y_pos,       0.0, 1.0,
            x_pos + w, y_pos,       1.0, 1.0,

            x_pos,     y_pos + h,   0.0, 0.0,
            x_pos + w, y_pos,       1.0, 1.0,
            x_pos + w, y_pos + h,   1.0, 0.0            
        ];

        let vao = gl.create_vertex_array().expect("Create VAO");
        let vbo = gl.create_buffer().expect("Create VBO");

        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        gl.bind_vertex_array( Some(vao) );
        
        gl.bind_buffer(glow::ARRAY_BUFFER, Some( vbo ));
        let u8_buffer = bytemuck::cast_slice(&vertices[..]);
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, u8_buffer, glow::STATIC_DRAW);

        gl.vertex_attrib_pointer_f32(
            0,              
            4,
            glow::FLOAT,
            false,
            0,
            0 );

        gl.enable_vertex_attrib_array(0);

        
        let texture = Texture::new(gl.clone(),"resources/textures/awesomeface.png",true);

        gl.bind_vertex_array(None);
                    
        // uncomment this call to draw in wireframe polygons.
        // gl.polygon_mode(glow::FRONT_AND_BACK, glow::LINE);




        event_loop.run(move |event, _, control_flow| {
            
            *control_flow = ControlFlow::WaitUntil( std::time::Instant::now() + std::time::Duration::from_millis(16));

            match event {
                Event::RedrawRequested(_) => {
                    // DRAW HERE
                    gl.clear_color(0.2, 0.3, 0.3, 1.0);
                    gl.clear(glow::COLOR_BUFFER_BIT);                    

                    // draw our first tria
                    shader.use_program();
                    shader.set_uniform_vec3( "textColor", &glm::vec3(1.0,1.0,1.0));


                    gl.bind_vertex_array(Some(vao));

                    gl.active_texture(glow::TEXTURE0);
                    texture.bind();

                    gl.draw_arrays(glow::TRIANGLES, 0, 6);

                    // no need to unbind it every time
                    gl.bind_vertex_array(None);
                    
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
                },
                _ => {}
            }
        } );
    }
}
