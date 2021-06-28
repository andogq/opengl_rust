use std::fs::read_to_string;
use std::ffi::{CString};

use cgmath::Matrix4;

mod individual_shader;
use individual_shader::*;

pub struct Shader {
    id: u32,
    built: bool,
    path: String
}

impl Shader {
    pub fn new(name: &str) -> Shader {
        let path = format!("./res/shaders/{}", name);

        Shader {
            id: 0,
            built: false,
            path
        }
    }

    pub fn build(&mut self) {
        // Load shaders from their respective files
        let vertex_shader_source = read_to_string(format!("{}/vertex.glsl", self.path)).expect("Problem reading shader");
        let fragment_shader_source = read_to_string(format!("{}/fragment.glsl", self.path)).expect("Problem reading shader");

        let vertex_shader = IndividualShader::new(ShaderType::Vertex, &vertex_shader_source);
        let fragment_shader = IndividualShader::new(ShaderType::Fragment, &fragment_shader_source);

        self.id = unsafe { gl::CreateProgram() };

        unsafe {
            // Attach the shaders
            gl::AttachShader(self.id, vertex_shader.id);
            gl::AttachShader(self.id, fragment_shader.id);

            // Link and check the program
            gl::LinkProgram(self.id);
            gl::ValidateProgram(self.id);
        }

        unsafe {
            // Should be done when they go out of scope
            gl::DeleteShader(vertex_shader.id);
            gl::DeleteShader(fragment_shader.id);
        }

        self.built = true;
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