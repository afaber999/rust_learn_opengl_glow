use glow::*;
use std::rc::Rc;
extern crate nalgebra_glm as glm;

// #[derive(Clone)]
pub struct TextureA {
//     pub id: u32,
//     pub type_: String,
//     pub path: String,
}

pub struct Mesh {
    gl : Rc<glow::Context>,

    // Render data
    vao: Option<glow::Buffer>,
    vbo_p: Option<glow::Buffer>,
    vbo_n: Option<glow::Buffer>,
    vbo_t: Option<glow::Buffer>,
    ebo: Option<glow::Buffer>,
    num_indices : usize, 
    num_vertices : usize, 
}

impl Mesh {

    pub fn new(
        gl : Rc<glow::Context>,
        positions: &[f32],
        normals: &[f32],
        tex_coords: &[f32],
        indices: &[u32] ) -> Self {

        unsafe {
            // Create buffer
            let vao = Some( gl.create_vertex_array().expect("Create VAO"));
            let vbo_p = Some( gl.create_buffer().expect("Create VBO"));
            let vbo_n = Some( gl.create_buffer().expect("Create VBO"));
            let vbo_t = Some( gl.create_buffer().expect("Create VBO"));
            let ebo = Some( gl.create_buffer().expect("Create EBO"));
            
            gl.bind_vertex_array( vao );
            
            // load positions
            gl.bind_buffer(glow::ARRAY_BUFFER, vbo_p );
            let u8_buffer = bytemuck::cast_slice(&positions);
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, u8_buffer, glow::STATIC_DRAW);

            gl.vertex_attrib_pointer_f32(
                0,
                3,
                glow::FLOAT,
                false,
                0,
                0);
            gl.enable_vertex_attrib_array(0);

            // load normals
            gl.bind_buffer(glow::ARRAY_BUFFER, vbo_n );
            let u8_buffer = bytemuck::cast_slice(&normals);
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, u8_buffer, glow::STATIC_DRAW);

            gl.vertex_attrib_pointer_f32(
                1,
                3,
                glow::FLOAT,
                false,
                0,
                0);
            gl.enable_vertex_attrib_array(1);

            // tex coord normals
            gl.bind_buffer(glow::ARRAY_BUFFER, vbo_t);
            let u8_buffer = bytemuck::cast_slice(&tex_coords);
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, u8_buffer, glow::STATIC_DRAW);

            gl.vertex_attrib_pointer_f32(
                2,
                2,
                glow::FLOAT,
                false,
                0,
                0);
            gl.enable_vertex_attrib_array(2);

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, ebo);
            let u8_buffer = bytemuck::cast_slice(&indices);
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, u8_buffer, glow::STATIC_DRAW);
            
            gl.bind_vertex_array( None );

            Self {
                gl,
                vao,
                vbo_p,
                vbo_n,
                vbo_t,
                ebo,
                num_indices :  indices.len(),
                num_vertices : positions.len(),
            }
        }
    }

    /// render the mesh
    pub fn draw(&self) {
        unsafe {
            // draw mesh
            self.gl.bind_vertex_array(self.vao);

            if self.num_indices > 0  {
                self.gl.draw_elements(
                    glow::TRIANGLES, 
                    self.num_indices as i32, 
                    glow::UNSIGNED_INT,
                    0);
            } else {
                self.gl.draw_arrays(glow::TRIANGLES, 0,  self.num_vertices as i32 / 3);
            }
            
            self.gl.bind_vertex_array(None);
        }
        // // bind appropriate textures
        // let mut diffuseNr  = 0;
        // let mut specularNr = 0;
        // let mut normalNr   = 0;
        // let mut heightNr   = 0;
        // for (i, texture) in self.textures.iter().enumerate() {
        //     gl::ActiveTexture(gl::TEXTURE0 + i as u32); // active proper texture unit before binding
        //     // retrieve texture number (the N in diffuse_textureN)
        //     let name = &texture.type_;
        //     let number = match name.as_str() {
        //         "texture_diffuse" => {
        //             diffuseNr += 1;
        //             diffuseNr
        //         },
        //         "texture_specular" => {
        //             specularNr += 1;
        //             specularNr
        //         }
        //         "texture_normal" => {
        //             normalNr += 1;
        //             normalNr
        //         }
        //         "texture_height" => {
        //             heightNr += 1;
        //             heightNr
        //         }
        //         _ => panic!("unknown texture type")
        //     };
            // now set the sampler to the correct texture unit
            //let sampler = CString::new(format!("{}{}", name, number)).unwrap();
            //gl::Uniform1i(gl::GetUniformLocation(shader.ID, sampler.as_ptr()), i as i32);
            // and finally bind the texture
            //gl::BindTexture(gl::TEXTURE_2D, texture.id);
        //}

        // always good practice to set everything back to defaults once configured.
        //gl::ActiveTexture(gl::TEXTURE0);
    }
}


impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            if let Some(id) = self.vbo_p {self.gl.delete_buffer(id);}
            if let Some(id) = self.vbo_n {self.gl.delete_buffer(id);}
            if let Some(id) = self.vbo_t {self.gl.delete_buffer(id);}
            if let Some(id) = self.ebo {self.gl.delete_buffer(id);}
            if let Some(id) = self.vao {self.gl.delete_buffer(id);}

        }
    }
}
