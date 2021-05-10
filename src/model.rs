use glow::*;
use std::ops::Drop;
use std::path::Path;
use crate::mesh::Mesh;
use std::rc::Rc;
//use texture::Texture;
use tobj;

pub struct Model {
    gl : Rc<glow::Context>,
    pub meshes : Vec<Mesh>,
    directory: String,    
}

impl Model {
    /// constructor, expects a filepath to a 3D model.
    pub fn new( gl : Rc<glow::Context>, path: &str) -> Self {
        
        let mut meshes = Vec::new();

        let path = Path::new(path);

        // retrieve the directory path of the filepath
        let directory = path.parent().unwrap_or_else(|| Path::new("")).to_str().unwrap().into();
        let obj = tobj::load_obj(path);

        let (models, materials) = obj.unwrap();
        for model in models {
            let mesh = &model.mesh;
            let num_vertices = mesh.positions.len() / 3;

            let new_mesh = Mesh::new(
                gl.clone(),
                &mesh.positions,
                &mesh.normals, 
                &mesh.texcoords,
                &[]);

            meshes.push(new_mesh);

            if let Some(material_id) = mesh.material_id {
                let material = &materials[material_id];     
                println!("material.diffuse_texture {} vertices {}", &material.diffuse_texture, num_vertices);
            }
        }

        Self {
            gl,
            meshes,
            directory,
            
        }
    }
}

impl Drop for Model {
    fn drop(&mut self) {

    }
}
    
    
    // #[derive(Debug, Default)]
// pub struct Model {
//     gl : Rc<glow::Context>,
//     pub meshes: Vec<Mesh>,
//     pub textures: Vec<Texture>,
//     directory: String,
// }


// impl Model {
//     /// constructor, expects a filepath to a 3D model.
//     pub fn new(path: &str) -> Self {
//         let mut model = Model::default();
//         model.load_model(path);
//         model
//     }

//     pub fn draw(&self, shader: &Shader) {
//         for mesh in &self.meshes {
//             unsafe { mesh.Draw(shader); }
//         }
//     }

//     // loads a model from file and stores the resulting meshes in the meshes vector.
//     fn load_model(&mut self, path: &str) {
//         let path = Path::new(path);

//         // retrieve the directory path of the filepath
//         self.directory = path.parent().unwrap_or_else(|| Path::new("")).to_str().unwrap().into();
//         let obj = tobj::load_obj(path);

//         let (models, materials) = obj.unwrap();
//         for model in models {
//             let mesh = &model.mesh;
//             let num_vertices = mesh.positions.len() / 3;

//             // data to fill
//             let mut vertices: Vec<Vertex> = Vec::with_capacity(num_vertices);
//             let indices: Vec<u32> = mesh.indices.clone();

//             let (p, n, t) = (&mesh.positions, &mesh.normals, &mesh.texcoords);
//             for i in 0..num_vertices {
//                 vertices.push(Vertex {
//                     Position:  vec3(p[i*3], p[i*3+1], p[i*3+2]),
//                     Normal:    vec3(n[i*3], n[i*3+1], n[i*3+2]),
//                     TexCoords: vec2(t[i*2], t[i*2+1]),
//                     ..Vertex::default()
//                 })
//             }

//             // process material
//             let mut textures = Vec::new();
//             if let Some(material_id) = mesh.material_id {
//                 let material = &materials[material_id];

//                 // 1. diffuse map
//                 if !material.diffuse_texture.is_empty() {
                    
//                     let texture = self.loadMaterialTexture(&material.diffuse_texture, "texture_diffuse");
//                     textures.push(texture);
//                 }
//                 // 2. specular map
//                 if !material.specular_texture.is_empty() {
//                     let texture = self.loadMaterialTexture(&material.specular_texture, "texture_specular");
//                     textures.push(texture);
//                 }
//                 // 3. normal map
//                 if !material.normal_texture.is_empty() {
//                     let texture = self.loadMaterialTexture(&material.normal_texture, "texture_normal");
//                     textures.push(texture);
//                 }
//                 // NOTE: no height maps
//             }

//             self.meshes.push(Mesh::new(vertices, indices, textures));
//         }

//     }

//     fn loadMaterialTexture(&mut self, path: &str, typeName: &str) -> Texture {
//         {
//             let texture = self.textures_loaded.iter().find(|t| t.path == path);
//             if let Some(texture) = texture {
//                 return texture.clone();
//             }
//         }

//         let texture = Texture {
//             id: unsafe { TextureFromFile(path, &self.directory) },
//             type_: typeName.into(),
//             path: path.into()
//         };
//         self.textures_loaded.push(texture.clone());
//         texture
//     }
// }

// unsafe fn TextureFromFile(path: &str, directory: &str) -> u32 {
//     let filename = format!("{}/{}", directory, path);

//     let mut textureID = 0;
//     gl::GenTextures(1, &mut textureID);

//     let img = image::open(&Path::new(&filename)).expect("Texture failed to load");
//     let img = img.flipv();
//     let format = match img {
//         ImageLuma8(_) => gl::RED,
//         ImageLumaA8(_) => gl::RG,
//         ImageRgb8(_) => gl::RGB,
//         ImageRgba8(_) => gl::RGBA,
//     };

//     let data = img.raw_pixels();

//     gl::BindTexture(gl::TEXTURE_2D, textureID);
//     gl::TexImage2D(gl::TEXTURE_2D, 0, format as i32, img.width() as i32, img.height() as i32,
//         0, format, gl::UNSIGNED_BYTE, &data[0] as *const u8 as *const c_void);
//     gl::GenerateMipmap(gl::TEXTURE_2D);

//     gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
//     gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
//     gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
//     gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

//     textureID
// }
