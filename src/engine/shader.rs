use std::fs::read_to_string;
use std::ffi::{CString};
use std::collections::HashMap;

use cgmath::Matrix4;

mod individual_shader;
use individual_shader::*;

pub struct Shader {
    id: u32,

    uniform_locations: HashMap<String, i32>
}

impl Shader {
    pub fn new(name: &str, geometry: bool) -> Shader {
        let path = format!("./res/shaders/{}", name);

        let mut shaders = Vec::new();

        // Load shaders from their respective files
        let vertex_shader_source = read_to_string(format!("{}.vert", path)).expect("Problem reading shader");
        let fragment_shader_source = read_to_string(format!("{}.frag", path)).expect("Problem reading shader");

        let vertex_shader = IndividualShader::new(ShaderType::Vertex, &vertex_shader_source);
        let fragment_shader = IndividualShader::new(ShaderType::Fragment, &fragment_shader_source);
        shaders.push(vertex_shader);
        shaders.push(fragment_shader);

        if geometry {
            let geometry_shader_source = read_to_string(format!("{}.geom", path)).expect("Problem reading shader");
            let geometry_shader = IndividualShader::new(ShaderType::Geometry, &geometry_shader_source);
            shaders.push(geometry_shader);
        }

        let id = unsafe { gl::CreateProgram() };

        // Attach the shaders
        for shader in shaders.iter() {
            unsafe { gl::AttachShader(id, shader.id) };
        }

        unsafe {
            // Link and check the program
            gl::LinkProgram(id);
            gl::ValidateProgram(id);
        }

        for shader in shaders.iter() {
            unsafe { gl::DeleteShader(shader.id) };
        }

        Shader {
            id,
            uniform_locations: HashMap::new()
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        };
    }

    pub fn get_uniform(&mut self, uniform_name: &str) -> i32 {
        match self.uniform_locations.get(uniform_name) {
            Some(uniform) => uniform.clone(),
            None => {
                // Bind the CString to a variable so it doesn't go out of scope
                let uniform_name_cstring = CString::new(uniform_name).expect("Invalid string to be converted to CString (might have null byte)");
        
                // Able to use the pointer here because it hasn't been freed, and return the location
                let uniform = unsafe { gl::GetUniformLocation(self.id, uniform_name_cstring.as_ptr()) };
        
                if uniform == -1 {
                    panic!("Uniform {} doesn't exist", uniform_name);
                }

                self.uniform_locations.insert(String::from(uniform_name), uniform);
        
                uniform
            }
        }
    }

    pub fn set_uniform(&mut self, uniform_name: &str, matrix: &Matrix4<f32>) {
        let uniform = self.get_uniform(uniform_name);
        let raw_matrix: [[f32; 4]; 4] = matrix.clone().into();

        unsafe { gl::UniformMatrix4fv(uniform, 1, gl::FALSE, raw_matrix[0].as_ptr()) };
    }
}