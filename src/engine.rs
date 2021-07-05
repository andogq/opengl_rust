mod shader;
pub mod model;
mod object;
mod camera;
mod renderer;

use shader::Shader;
use model::Model;
use object::Object;
use camera::Camera;
use renderer::Renderer;

use cgmath::Vector3;

use crate::logger::{ Logger, Level };

pub struct Engine {
    shaders: Vec<Shader>,
    models: Vec<Model>,
    cameras: Vec<Camera>,
    renderer: Renderer,

    initialised: bool,
    logger: Logger
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            shaders: Vec::new(),
            models: Vec::new(),
            cameras: Vec::new(),
            renderer: Renderer::new(),

            initialised: false,
            logger: Logger::new(Level::Debug)
        }
    }

    pub fn init(&mut self) {
        // Initialise the renderer
        self.renderer.init();

        self.initialised = true;
        self.logger.info("Engine finished initialising");
    }

    pub fn add_shader(&mut self, name: &str, geometry: bool) -> usize {
        self.logger.info(&format!("Adding shader `{}`", name));

        let index = self.shaders.len();
        self.shaders.push(Shader::new(name, geometry));

        index
    }

    pub fn add_model(&mut self, model: Model) -> usize {
        self.logger.info("Adding model");

        let index = self.models.len();

        self.models.push(model);

        index
    }

    pub fn add_object(&mut self, model: usize, position: Vector3<f32>, scale: Vector3<f32>) {
        self.logger.info("Adding object");

        let object = Object::new(model, position, scale);
        self.models[model].add_object(object);
    }

    pub fn add_camera(&mut self, position: Vector3<f32>, rotation: Vector3<f32>, aspect: f32, fov: f32) -> usize {
        self.logger.info("Adding camera");
        
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