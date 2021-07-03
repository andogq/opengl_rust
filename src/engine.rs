mod shader;
mod model;
mod object;
mod camera;
mod renderer;

use shader::Shader;
use model::Model;
use object::Object;
use camera::Camera;
use renderer::Renderer;

use std::collections::HashMap;

use cgmath::Vector3;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

pub struct Engine {
    shaders: Vec<Shader>,
    models: Vec<Model>,
    objects: Vec<Object>,
    cameras: HashMap<String, Camera>,
    renderer: Renderer,

    initialised: bool
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            shaders: Vec::new(),
            models: Vec::new(),
            objects: Vec::new(),
            cameras: HashMap::new(),
            renderer: Renderer::new(),

            initialised: false
        }
    }

    pub fn init(&mut self) {
        // Initialise the renderer
        self.renderer.init();

        self.initialised = true;
        println!("Engine finished initialising");
    }

    pub fn add_shader(&mut self, name: &str) -> usize {
        println!("Adding shader `{}`", name);

        let index = self.shaders.len();
        self.shaders.push(Shader::new(name));

        index
    }

    pub fn add_model(&mut self, points: &[f32], indices: &[u32], shader: usize) -> usize {
        let model = Model::new(points, indices, shader);

        let index = self.models.len();

        self.models.push(model);

        index
    }

    pub fn add_object(&mut self, model: usize, position: Vector3<f32>, scale: Vector3<f32>) {
        println!("Adding object");

        self.objects.push(Object::new(model, position, scale));
    }

    pub fn add_camera(&mut self, name: &str, position: Vector3<f32>, rotation: Vector3<f32>, aspect: f32, fov: f32) {
        self.cameras.entry(String::from(name)).or_insert(Camera::new(position, rotation, aspect, fov));
    }

    pub fn render(&mut self, camera: &str) {
        let camera = self.cameras.get(camera).unwrap();

        self.renderer.render(&camera.vp_matrix(), &self.objects, &self.models, &self.shaders);
    }
}