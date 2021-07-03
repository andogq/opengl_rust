use std::ffi::{CStr};
use std::time::{Instant, Duration};

use cgmath::{Matrix4};

use super::object::Object;
use super::model::Model;
use super::shader::Shader;

pub struct Renderer {
    initialised: bool,

    fps: u32,
    last_draw: Instant
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            initialised: false,

            fps: 60,
            last_draw: Instant::now()
        }
    }

    pub fn init(&mut self) {
        // Assumes OpenGL bindings have been setup

        // Get OpenGL version
        unsafe {
            let data = gl::GetString(gl::VERSION);
            println!("{}", String::from_utf8(CStr::from_ptr(data as *const i8).to_bytes().to_vec()).unwrap());
        };

        // Set the clear color
        unsafe { gl::ClearColor(0.0, 0.0, 0.0, 1.0) };

        self.initialised = true;
        check_errors();
        println!("Renderer finished initialising");
    }

    pub fn set_fps(&mut self, fps: u32) {
        self.fps = fps;
    }

    pub fn render(&self, vp_matrix: &Matrix4<f32>, objects: &Vec<Object>, models: &Vec<Model>, shaders: &Vec<Shader>) {
        // Clear the screen        
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };

        for object in objects.iter() {
            let model = &models[object.get_model()];
            let shader = &shaders[model.get_shader()];

            // Bind the shader
            shader.bind();
            model.get_vertex_array().bind();

            let uniform = shader.get_uniform("u_mvp_matrix");

            let mvp_matrix: [[f32; 4]; 4] = (vp_matrix * object.model_matrix()).into();
            unsafe { gl::UniformMatrix4fv(uniform, 1, gl::FALSE, mvp_matrix[0].as_ptr()) };

            unsafe { gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null()) };
        }

        check_errors();
    }

    pub fn ready_to_render(&self) -> (bool, Instant) {
        // Calculate when next frame should be drawn, and trigger a draw call or wait
        let next_draw = self.last_draw + Duration::from_millis(1000 / self.fps as u64);
        
        return (next_draw <= Instant::now(), next_draw);
    }
}

fn check_errors() {
    loop {
        let error = unsafe { gl::GetError() };

        if error != 0 { println!("[!] OpenGL Error: 0x{:x}", error); }
        else { break; }
    }
}