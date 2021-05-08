use glow::*;
extern crate nalgebra_glm as glm;
use std::rc::Rc;
use std::collections::HashMap;
use std::cell::RefCell;
use core::convert::TryInto;
use std::ops::Drop;

#[derive(Debug)]
pub struct Shader {
    gl : Rc<glow::Context>,
    program: Option<glow::Program>,
    uniform_lookup: RefCell<HashMap<String, glow::UniformLocation>>,
}

impl Shader {
    /// Create new shader form two strings
    /// ------------------------------------------------------------------------
    pub fn new_from_strings(gl : Rc<glow::Context>, vx_shader:&str, fg_shader:&str )-> Self{

        unsafe {

            let shader_sources = [
                (glow::VERTEX_SHADER, vx_shader),
                (glow::FRAGMENT_SHADER, fg_shader),
            ];
        
            let mut shaders = Vec::with_capacity(shader_sources.len());
        
            // compile the shaders
            let program = gl.create_program().expect("Cannot create program");
           
            for (shader_type, shader_source) in shader_sources.iter() {
                let shader = gl
                    .create_shader(*shader_type)
                    .expect(&format!( "Cannot create shader: {}", shader_source));

                gl.shader_source(shader, shader_source);
                gl.compile_shader(shader);
    
                // println!("Compiling {}", shader_source );
                if !gl.get_shader_compile_status(shader) {
                    panic!( "{}", gl.get_shader_info_log(shader));
                }
                gl.attach_shader(program, shader);
                shaders.push(shader);
            }
    
            // link the shaders
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!( "{}",gl.get_program_info_log(program));
            }
    
            for shader in shaders {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }

            Self {
                gl : gl,
                program : Some(program),
                uniform_lookup: RefCell::new(HashMap::new()),
            }
        }
    }    

    /// Create new shader form two files
    /// ------------------------------------------------------------------------
    pub fn new_from_files( gl:Rc<glow::Context>, vx_shader_path:&str, fg_shader_path:&str )-> Self {

        let vx_shader = std::fs::read_to_string(vx_shader_path).expect(format!("Failed to read file: {}",vx_shader_path).as_str());
        let fg_shader = std::fs::read_to_string(fg_shader_path).expect(format!("Failed to read file: {}",fg_shader_path).as_str());

        Shader::new_from_strings(gl, vx_shader.as_str(), fg_shader.as_str())
    }
 
    /// activate the shader
    /// ------------------------------------------------------------------------
    pub fn use_program(&self) {
        unsafe{ self.gl.use_program(self.program); }
    }

    /// Set uniform as 3 floats
    /// ------------------------------------------------------------------------
    pub fn set_uniform_3_f32(&self,  field:&str, vx:f32, vy:f32, vz:f32) {

        if let Some(pgm) = self.program {
            unsafe {
                let mut uniform_lookup = self.uniform_lookup.borrow_mut();
                let location = uniform_lookup.get(field);
                if location.is_none() {
                    let location = self.gl.get_uniform_location(pgm, field);
                    self.gl.uniform_3_f32(location.as_ref(), vx, vy, vz);
                    if let Some( loc ) = location {
                        uniform_lookup.insert(field.into(), loc);
                    } 
                } else {
                    self.gl.uniform_3_f32(location, vx, vy, vz);
                }
            }    
        }
    }

    /// Set uniform as 4 floats
    /// ------------------------------------------------------------------------
    pub fn set_uniform_4_f32(&self,  field:&str, vx:f32, vy:f32, vz:f32, vw:f32) {

        if let Some(pgm) = self.program {
            unsafe {
                let mut uniform_lookup = self.uniform_lookup.borrow_mut();
                let location = uniform_lookup.get(field);
                if location.is_none() {
                    let location = self.gl.get_uniform_location(pgm, field);
                    self.gl.uniform_4_f32(location.as_ref(), vx, vy, vz, vw);
                    if let Some( loc ) = location {
                        uniform_lookup.insert(field.into(), loc);
                    } 
                } else {
                    self.gl.uniform_4_f32(location, vx, vy, vz, vw);
                }
            }    
        }
    }

    /// Set uniform as f32
    /// ------------------------------------------------------------------------    
    pub fn set_uniform_f32(&self,  field:&str, value: f32) {

        if let Some(pgm) = self.program {
            unsafe {
                let mut uniform_lookup = self.uniform_lookup.borrow_mut();
                let location = uniform_lookup.get(field);
                if location.is_some() {
                    self.gl.uniform_1_f32(location, value);
                } else {
                    let location =  self.gl.get_uniform_location(pgm, field);
                    self.gl.uniform_1_f32(location.as_ref(), value);
                    if let Some( loc ) = location {
                        uniform_lookup.insert(field.into(), loc);
                    } 
                }
            }    
        }
    }

    /// Set uniform as i32
    /// ------------------------------------------------------------------------    
    pub fn set_uniform_i32(&self,  field:&str, value: i32) {

        if let Some(pgm) = self.program {
            unsafe {
                let mut uniform_lookup = self.uniform_lookup.borrow_mut();
                let location = uniform_lookup.get(field);
                if location.is_some() {
                    //println!("Has key for i32 {} {:?}", field, location);
                    self.gl.uniform_1_i32(location, value);
                } else {
                    let location =  self.gl.get_uniform_location(pgm, field);
                    self.gl.uniform_1_i32(location.as_ref(), value);
                    if let Some( loc ) = location {
                        //println!("inser key {} location {} for i32", field, loc);
                        uniform_lookup.insert(field.into(), loc);
                    } 
                }
            }    
        }
    }

    /// Set uniform as vec3
    /// ------------------------------------------------------------------------
    pub fn set_uniform_vec3(&self,  field:&str, value: &glm::Vec3) {
        self.set_uniform_3_f32( field, value.x, value.y, value.z);
    }

    /// Set uniform as vec4
    /// ------------------------------------------------------------------------
    pub fn set_uniform_vec4(&self,  field:&str, value: &glm::Vec4) {
        self.set_uniform_4_f32( field, value.x, value.y, value.z, value.w);
    }

    /// Set uniform as Mat4
    /// ------------------------------------------------------------------------
    pub fn set_uniform_mat4(&self,  field:&str, value: &glm::Mat4) {

        if let Some(pgm) = self.program {
            unsafe {
                let mut uniform_lookup = self.uniform_lookup.borrow_mut();
                let location = uniform_lookup.get(field);
                if location.is_some() {
                    //println!("Has key {} {:?}", field, location);
                    self.gl.uniform_matrix_4_f32_slice(location, false, value.as_slice().try_into().unwrap());
                } else {
                    let location =  self.gl.get_uniform_location(pgm, field);
                    self.gl.uniform_matrix_4_f32_slice(location.as_ref(), false, value.as_slice().try_into().unwrap());
                    if let Some( loc ) = location {
                        //println!("inser key {} location {}", field, loc);
                        uniform_lookup.insert(field.into(), loc);
                    } 
                }
            }    
        }
    }
}


impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            if let Some(id) = self.program {
                self.gl.delete_program(id);
            }
        }
    }
}