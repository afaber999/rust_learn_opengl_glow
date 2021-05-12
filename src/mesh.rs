use glow::*;
use std::rc::Rc;
use crate::texture;
use crate::shader::Shader;
extern crate nalgebra_glm as glm;

pub enum MeshTexture {
    DiffuseMap(Rc<texture::Texture>),
    SpecularMap(Rc<texture::Texture>),
    NormalMap(Rc<texture::Texture>),
}

pub struct Mesh {
    gl : Rc<glow::Context>,

    // Render data
    vao: Option<glow::Buffer>,
    vbo_p: Option<glow::Buffer>,
    vbo_n: Option<glow::Buffer>,
    vbo_t: Option<glow::Buffer>,
    ebo: Option<glow::Buffer>,
    textures: Vec<MeshTexture>,
    num_indices : usize, 
    num_vertices : usize, 
}

impl Mesh {

    pub fn new(
        gl : Rc<glow::Context>,
        positions: &[f32],
        normals: &[f32],
        tex_coords: &[f32],
        indices: &[u32],
        textures: Vec<MeshTexture> ) -> Self {

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
            // println!("Vertices {} Indices {}", positions.len(), indices.len());
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
                textures,
                num_indices :  indices.len(),
                num_vertices : positions.len(),
            }
        }
    }

    /// render the mesh
    pub fn draw(&self, shader :&Shader) {
        unsafe {
            // bind vao
            self.gl.bind_vertex_array(self.vao);

            let mut diffuse_id = 0;
            let mut specular_id = 0;
            let mut normal_id = 0;

            // set textures
            for (idx, mesh_texture) in self.textures.iter().enumerate() {
                let (texture,field) = match mesh_texture {
                    MeshTexture::DiffuseMap(texture) => {
                        diffuse_id += 1;
                        (texture, format!("texture_diffuse{}", diffuse_id) )
                    }
                    MeshTexture::SpecularMap(texture) => {
                        specular_id += 1;
                        (texture, format!("texture_specular{}", specular_id))
                    }
                    MeshTexture::NormalMap(texture) => {
                        normal_id += 1;
                        (texture, format!("texture_normal{}", normal_id))
                    }
                };
                //println!("Set mesh  texture {} to {}", &field, idx);
                self.gl.active_texture(glow::TEXTURE0 + idx as u32);
                shader.set_uniform_i32(&field, idx as i32);
                texture.bind();
            }

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
