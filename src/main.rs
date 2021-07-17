mod window;
mod engine;

use cgmath::Vector3;
use window::Window;
use engine::{Camera, Engine, models, traits::{ Renderable }};

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

const PI : f32 = 3.141592653589793;

struct State {
    count: u32
}

fn test_update(global_state: &mut State, objects: &mut Vec<Box<dyn Renderable>>) {
    for object in objects.iter_mut() {
        global_state.count += 1;
        object.rotate(Vector3::new(0.01, 0.0, 0.0));

        println!("Count is: {}", global_state.count);
    }
}

fn main() {
    let window = Window::new();

    let mut engine = Engine::new(State {
        count: 0
    });

    let camera = Camera::new(PI / 3.0, (WINDOW_WIDTH as f32) / (WINDOW_HEIGHT as f32), 0.1, 1000.0);
    engine.use_camera(&camera);

    // let square = models::Square::new();
    // engine.add_object(&square);

    let cube = models::Cube::new();
    engine.add_object(Box::new(cube));

    engine.add_update(&test_update);
    
    window.run(|pressed| {
        engine.update();
        
        for key in pressed.iter() {
            match key {
                _ => ()
            }
        }
    });
}