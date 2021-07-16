use gl::types::*;

use cgmath::Vector3;

mod window;
mod engine;

use window::Window;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

const PI : f32 = 3.141592653589793;

fn main() {
    let window = Window::new();
    
    window.run(move |pressed| {
        for key in pressed.iter() {
            match key {
                _ => ()
            }
        }
    });
}