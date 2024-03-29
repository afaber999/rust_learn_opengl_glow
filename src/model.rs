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
            return rc_texture.clone();
        }
        let rc_texture = Rc::new( Texture::new(self.gl.clone(), &filename,true));
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
        let mut load_options = tobj::LoadOptions::default();
        load_options.triangulate = true;
        load_options.single_index = true;

        let obj = tobj::load_obj(path, &load_options);

        let (models, materials_result) = obj.unwrap();
        
        for model in models.into_iter(){

            let mesh = &model.mesh;
            
            let mut textures:Vec<MeshTexture>= Vec::new();

            if let Some(material_id) = mesh.material_id {
                if let Ok(materials) = &materials_result {
                    let material = &materials[material_id];
                    if let Some( diffuse ) = &material.diffuse_texture {
                        println!("material.diffuse_texture {}", diffuse);
                        let filename = format!("{}/{}", &directory, diffuse);
                        let rc_texture = texture_pool.load_texture(&filename);
                        textures.push( MeshTexture::DiffuseMap( rc_texture ));
                    }
                    if let Some( specular ) = &material.specular_texture {
                        println!("material.normal_texture {}", specular);
                        let filename = format!("{}/{}", &directory, specular);
                        let rc_texture = texture_pool.load_texture(&filename);
                        textures.push( MeshTexture::SpecularMap( rc_texture ));
                    }
    
                    if let Some( normal ) = &material.normal_texture {
                        println!("material.normal_texture {}", normal);
                        let filename = format!("{}/{}", &directory, normal);
                        let rc_texture = texture_pool.load_texture(&filename);
                        textures.push( MeshTexture::NormalMap( rc_texture ));
                    }
                }
            }

            // println!("POSITIONS");
            // for (idx, tc) in mesh.positions.iter().enumerate() {
            //     if idx%3 == 0 {
            //         print!("\npos: {} ", idx);
            //     } 
            //     print!(" {}", tc );
            // }

            // println!("TEXCOORDS");
            // for (idx, tc) in mesh.texcoords.iter().enumerate() {
            //     if idx%2 == 0 {
            //         print!("\nTEX: {} ", idx);
            //     }
            //     print!(" {}", tc );
            // }

            // println!("INDICES");
            // for idx in mesh.indices.iter() {
            //     print!(" {}", idx );
            // }

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
