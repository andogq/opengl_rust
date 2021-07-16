use gl::types::*;

use cgmath::{ Rad, Vector3, Matrix4 };

mod window;
mod engine;

use window::Window;
use engine::{ Engine, Shader, Camera, VertexArray, Layout, traits, models };

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

const PI : f32 = 3.141592653589793;

fn main() {
    let window = Window::new();

    let mut engine = Engine::new();

    let camera = Camera::new(PI / 3.0, (WINDOW_WIDTH as f32) / (WINDOW_HEIGHT as f32), 0.1, 1000.0);
    engine.use_camera(&camera);

    let square = models::Square::new();
    engine.add_object(&square);
    
    window.run(|pressed| {
        engine.update();
        
        for key in pressed.iter() {
            match key {
                _ => ()
            }
        }
    });
}