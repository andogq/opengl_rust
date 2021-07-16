use gl::types::*;

use cgmath::Vector3;

mod window;
mod engine;

use window::Window;
use engine::Engine;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

const PI : f32 = 3.141592653589793;

fn main() {
    let window = Window::new();

    let engine = Engine::new();
    
    window.run(move |pressed| {
        engine.update();
        
        for key in pressed.iter() {
            match key {
                _ => ()
            }
        }
    });
}