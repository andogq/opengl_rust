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

use cgmath::Vector3;

pub struct Engine {
    shaders: Vec<Shader>,
    models: Vec<Model>,
    cameras: Vec<Camera>,
    renderer: Renderer,

    initialised: bool
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            shaders: Vec::new(),
            models: Vec::new(),
            cameras: Vec::new(),
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

    pub fn add_shader(&mut self, name: &str, geometry: bool) -> usize {
        println!("Adding shader `{}`", name);

        let index = self.shaders.len();
        self.shaders.push(Shader::new(name, geometry));

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

        let object = Object::new(model, position, scale);
        self.models[model].add_object(object);
    }

    pub fn add_camera(&mut self, position: Vector3<f32>, rotation: Vector3<f32>, aspect: f32, fov: f32) -> usize {
        let index = self.cameras.len();

        self.cameras.push(Camera::new(position, rotation, aspect, fov));

        index
    }

    pub fn get_camera(&mut self, camera_id: usize) -> &mut Camera {
        &mut self.cameras[camera_id]
    }

    pub fn render(&mut self, camera: usize) {
        let camera = &self.cameras[camera];

        self.renderer.render(&camera.get_view_matrix(), &camera.get_projection_matrix(), &self.models, &mut self.shaders);
    }
}