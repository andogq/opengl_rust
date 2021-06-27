use std::fs::read_to_string;
use std::ffi::{CString};

use cgmath::Matrix4;

mod shader;
use shader::*;

pub struct Program {
    pub id: u32
}

impl Program {
    pub fn new(name: &str) -> Program {
        // Load shaders from their respective files
        let path = format!("./res/shaders/{}", name);

        let vertex_shader_source = read_to_string(format!("{}/vertex.glsl", path)).expect("Problem reading shader");
        let fragment_shader_source = read_to_string(format!("{}/fragment.glsl", path)).expect("Problem reading shader");

        let vertex_shader = Shader::new(ShaderType::Vertex, &vertex_shader_source);
        let fragment_shader = Shader::new(ShaderType::Fragment, &fragment_shader_source);

        let id = unsafe { gl::CreateProgram() };

        unsafe {
            // Attach the shaders
            gl::AttachShader(id, vertex_shader.id);
            gl::AttachShader(id, fragment_shader.id);

            // Link and check the program
            gl::LinkProgram(id);
            gl::ValidateProgram(id);
        }

        unsafe {
            // Should be done when they go out of scope
            gl::DeleteShader(vertex_shader.id);
            gl::DeleteShader(fragment_shader.id);
        }

        Program {
            id
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        };
    }

    pub fn get_uniform(&self, uniform_name: &str) -> i32 {
        // Bind the CString to a variable so it doesn't go out of scope
        let uniform_name_cstring = CString::new(uniform_name).expect("Invalid string to be converted to CString (might have null byte)");

        // Able to use the pointer here because it hasn't been freed, and return the location
        let location = unsafe { gl::GetUniformLocation(self.id, uniform_name_cstring.as_ptr()) };

        if location == -1 {
            panic!("Uniform {} doesn't exist", uniform_name);
        }

        location
    }

    pub fn set_uniform(&self, uniform_name: &str, matrix: &Matrix4<f32>) {

    }
}