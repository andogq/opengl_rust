pub mod traits;
mod vertex_array;
mod shader;
mod camera;
mod renderer;

use traits::*;
use vertex_array::VertexArray;
use shader::Shader;
use camera::Camera;
use renderer::Renderer;

pub struct Engine {
    objects: Vec<Box<dyn Renderable>>,
    cameras: Vec<Box<Camera>>,
    lights: Vec<Box<dyn Light>>,
    shaders: Vec<Box<Shader>>,
    active_camera: Option<Box<Camera>>,
    renderer: Renderer
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            objects: Vec::new(),
            cameras: Vec::new(),
            lights: Vec::new(),
            shaders: Vec::new(),
            active_camera: None,
            renderer: Renderer::new()
        }
    }

    pub fn update(&self) {
        if let Some(camera) = &self.active_camera {
            self.renderer.render(camera, &self.objects, &self.lights);
        } else { eprintln!("No camera is selected"); }
    }
}