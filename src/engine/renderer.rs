use std::ffi::{CStr};

use cgmath::{Matrix4};

use super::model::Model;
use super::shader::Shader;

use crate::logger::{ Logger, Level };

pub struct Renderer {
    initialised: bool,
    logger: Logger
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            initialised: false,
            logger: Logger::new(Level::Debug)
        }
    }

    pub fn init(&mut self) {
        self.logger.info("Initialising renderer");
        // Assumes OpenGL bindings have been setup

        // Get OpenGL version
        unsafe {
            let data = gl::GetString(gl::VERSION);
            self.logger.debug(&format!("OpenGL Version: {}", String::from_utf8(CStr::from_ptr(data as *const i8).to_bytes().to_vec()).unwrap()));
        };

        // Set the clear color
        unsafe { gl::ClearColor(1.0, 1.0, 1.0, 1.0) };

        self.initialised = true;
        self.check_errors();
    }

    pub fn render(&self, view_matrix: &Matrix4<f32>, projection_matrix: &Matrix4<f32>, models: &Vec<Model>, shaders: &mut Vec<Shader>) {
        // Clear the screen        
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) };
        
        unsafe { gl::Enable(gl::DEPTH_TEST) };

        for model in models.iter() {
            let shader = &mut shaders[model.get_shader()];

            // Bind the shader
            shader.bind();
            model.get_vertex_array().bind();

            for object in model.get_objects().iter() {
                shader.set_uniform("u_model_matrix", &object.model_matrix());
                shader.set_uniform("u_view_matrix", view_matrix);
                shader.set_uniform("u_projection_matrix", projection_matrix);
    
                unsafe { gl::PointSize(10.0) };
                unsafe { gl::DrawElements(gl::TRIANGLES, model.get_vertex_array().get_index_length() as i32, gl::UNSIGNED_INT, std::ptr::null()) };
            }
        }

        self.check_errors();
    }

    fn check_errors(&self) {
        loop {
            let error = unsafe { gl::GetError() };
    
            if error != 0 { self.logger.error(&format!("[!] OpenGL Error: 0x{:x}", error)); }
            else { break; }
        }
    }
}
