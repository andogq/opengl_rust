use super::{Camera, Light, Renderable, traits::WorldPosition};

use gl::types::*;
use std::ffi::{CStr, c_void};

pub struct Renderer {

}

impl Renderer {
    pub fn new() -> Renderer {
        // Get OpenGL version
        unsafe {
            let data = gl::GetString(gl::VERSION);
            println!("OpenGL Version: {}", String::from_utf8(CStr::from_ptr(data as *const i8).to_bytes().to_vec()).unwrap());
        };

        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::DebugMessageCallback(Some(debug_callback), std::ptr::null());
        };

        Renderer {

        }
    }

    pub fn render(&self, camera: &Camera, objects: &[Box<dyn Renderable>], lights: &[Box<dyn Light>]) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);
        }

        for object in objects.iter() {
            let shader = object.get_shader();
            let vertex_array = object.get_vertex_array();
            
            shader.bind();
            vertex_array.bind();

            shader.set_uniform("u_model_matrix", object.get_model_matrix());
            shader.set_uniform("u_view_matrix", camera.get_model_matrix());
            shader.set_uniform("u_projection_matrix", camera.get_projection_matrix());

            unsafe { gl::PointSize(10.0) };
            unsafe { gl::DrawElements(gl::TRIANGLES, vertex_array.get_index_length() as i32, gl::UNSIGNED_INT, std::ptr::null()) }
        }
    }
}

extern "system"
fn debug_callback(_source: GLenum, _type: GLenum, _id: GLuint, severity: GLenum, _length: GLsizei, message: *const GLchar, _user_param: *mut c_void) {
    let message = unsafe { CStr::from_ptr(message) }.to_str().unwrap();

    eprintln!("OpenGL Error: {} {} {}", _type, severity, message);
}