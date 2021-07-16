pub mod traits;
mod vertex_array;
mod shader;
mod camera;
mod renderer;

use traits::*;
use vertex_array::VertexArray;
pub use shader::Shader;
pub use camera::Camera;
use renderer::Renderer;

pub struct Engine<'a> {
    objects: Vec<Box<dyn Renderable>>,
    lights: Vec<Box<dyn Light>>,
    active_camera: Option<&'a Camera>,
    renderer: Renderer
}

impl<'a> Engine<'a> {
    pub fn new() -> Engine<'a> {
        Engine {
            objects: Vec::new(),
            lights: Vec::new(),
            active_camera: None,
            renderer: Renderer::new()
        }
    }

    pub fn use_camera(&mut self, camera: &'a Camera) {
        self.active_camera = Some(camera);
    }

    pub fn update(&self) {
        if let Some(camera) = self.active_camera {
            self.renderer.render(camera, &self.objects, &self.lights);
        } else { eprintln!("No camera is selected"); }
    }
}