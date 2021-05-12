use std::ops::Drop;
use std::path::Path;
use std::rc::Rc;
use std::collections::HashMap;

use crate::{mesh::{Mesh, MeshTexture}, shader::Shader, texture::Texture};
use tobj;


pub struct TexturePool {
    gl : Rc<glow::Context>,    
    texture_pool : HashMap<String, Rc<Texture>>,
}

impl TexturePool {
    pub fn new(gl : Rc<glow::Context>) -> Self {
        Self {
            gl,
            texture_pool : HashMap::new(),
        }
    }

    pub fn load_texture(&mut self, filename :&str) ->Rc<Texture> {
        if let Some(rc_texture) = self.texture_pool.get(filename.into()) {
            println!("Found texture in pool: {}", filename);
            return rc_texture.clone();
        }
        println!("Load texture into pool: {}", filename);
        let rc_texture = Rc::new( Texture::new(self.gl.clone(), &filename));
        self.texture_pool.insert(filename.into(), rc_texture.clone());
        return rc_texture;
    }
}

pub struct Model {
    meshes : Vec<Mesh>,
}

impl Model {
    /// constructor, expects a filepath to a 3D model.
    pub fn new( gl : Rc<glow::Context>, path: &str) -> Self {
        
        let mut meshes = Vec::new();
        let mut texture_pool = TexturePool::new(gl.clone());

        let path = Path::new(path);

        // retrieve the directory path of the filepath
        let directory : String = path.parent().unwrap_or_else(|| Path::new("")).to_str().unwrap().into();
        let obj = tobj::load_obj(path, false);

        let (models, materials) = obj.unwrap();
        
        for model in models.into_iter() {

            let mesh = &model.mesh;

            let mut textures:Vec<MeshTexture>= Vec::new();

            if let Some(material_id) = mesh.material_id {
                let material = &materials[material_id];     
                if material.diffuse_texture.len() > 0 {
                    println!("material.diffuse_texture {}", &material.diffuse_texture);
                    let filename = format!("{}/{}", &directory, material.diffuse_texture);
                    let rc_texture = texture_pool.load_texture(&filename);
                    textures.push( MeshTexture::DiffuseMap( rc_texture ));
                }

                if material.specular_texture.len() > 0 {
                    println!("material.normal_texture {}", &material.specular_texture);
                    let filename = format!("{}/{}", &directory, material.specular_texture);
                    let rc_texture = texture_pool.load_texture(&filename);
                    textures.push( MeshTexture::SpecularMap( rc_texture ));
                }

                if material.normal_texture.len() > 0 {
                    println!("material.normal_texture {}", &material.normal_texture);
                    let filename = format!("{}/{}", &directory, material.normal_texture);
                    let rc_texture = texture_pool.load_texture(&filename);
                    textures.push( MeshTexture::NormalMap( rc_texture ));
                }
            }

            let new_mesh = Mesh::new(
                gl.clone(),
                &mesh.positions,
                &mesh.normals, 
                &mesh.texcoords,
                &mesh.indices,
                textures);

            meshes.push(new_mesh);
        }

        Self {
            meshes,
        }
    }
    pub fn draw(&self, shader: &Shader) {
        for mesh in &self.meshes {
            mesh.draw(shader);
        }
    }
}

impl Drop for Model {
    fn drop(&mut self) {

    }
}
