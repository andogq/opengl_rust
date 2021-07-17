pub mod traits;
mod vertex_array;
mod shader;
mod camera;
mod renderer;
pub mod models;

use traits::*;
pub use vertex_array::{ VertexArray, Layout };
pub use shader::Shader;
pub use camera::Camera;
use renderer::Renderer;

pub struct Engine<'a> {
    objects: Vec<Box<dyn Renderable>>,
    lights: Vec<Box<dyn Light>>,
    active_camera: Option<&'a Camera>,
    renderer: Renderer,
    updates: Vec<&'a dyn Fn(&mut Vec<Box<dyn Renderable>>)>
}

impl<'a> Engine<'a> {
    pub fn new() -> Engine<'a> {
        Engine {
            objects: Vec::new(),
            lights: Vec::new(),
            active_camera: None,
            renderer: Renderer::new(),
            updates: Vec::new(),
        }
    }

    pub fn use_camera(&mut self, camera: &'a Camera) {
        self.active_camera = Some(camera);
    }

    pub fn add_object(&mut self, object: Box<dyn Renderable>) {
        self.objects.push(object);
    }

    pub fn add_update(&mut self, update: &'a dyn Fn(&mut Vec<Box<dyn Renderable>>)) {
        self.updates.push(update);
    }

    pub fn update(&mut self) {
        for update in self.updates.iter() {
            update(&mut self.objects);
        }

        if let Some(camera) = self.active_camera {
            self.renderer.render(camera, &self.objects, &self.lights);
        } else { eprintln!("No camera is selected"); }
    }
}