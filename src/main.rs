use gl::types::*;

use cgmath::Vector3;

mod window;
use window::Window;

mod engine;
use engine::{Engine, traits::WorldPosition};

mod logger;
use logger::{ Logger, Level };

mod models;

use models::NewSqaure;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

const PI : f32 = 3.141592653589793;

fn main() {
    let log = Logger::new(Level::Debug);
    
    log.info("Starting program");

    let window = Window::new();
    
    let mut engine = Engine::new();

    engine.init();

    let square = NewSqaure::new();
    engine.add_renderable(square);

    let lighting_shader = engine.add_shader("lighting", true);
    let red_shader = engine.add_shader("red", false);

    // let cube_model = engine.add_model(models::cube::new(lighting_shader));
    // engine.add_object(cube_model, Vector3::new(0.0, 0.0, 0.0), Vector3::new(100.0, 100.0, 100.0));
    
    // let red_cube_model = engine.add_model(models::cube::new(red_shader));
    // engine.add_object(red_cube_model, Vector3::new(0.0, 500.0, -500.0), Vector3::new(5.0, 5.0, 5.0));

    // let square_model = engine.add_model(models::square::new(
    //     &[
    //         [[0.0, 1.0, -1.0], [1.0, 1.0, 0.0]],
    //         [[0.0, 0.0, 3.0], [1.0, 0.0, 1.0]]
    //     ],
    //     red_shader
    // ));
    // engine.add_object(square_model, Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));

    let plane = models::plane::Plane::new(100);
    let plane_model = engine.add_model(plane.to_model(lighting_shader));
    
    engine.add_object(plane_model, Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));

    let main_camera = engine.add_camera(Vector3::new(-50.0, -5.0, -50.0), Vector3::new(0.0, 3.0 / 4.0 * PI, 0.0), (WINDOW_WIDTH as f32)/(WINDOW_HEIGHT as f32), PI/2.0);

    let step = 0.1;
    let rstep = 0.05;

    window.run(move |pressed| {
        let camera = engine.get_camera(main_camera);
        
        let pitch = camera.get_rotation().x;
        let yaw = camera.get_rotation().y;

        let xz_length = pitch.cos() * step;
        let movement: Vector3<f32> = Vector3::new(-xz_length * yaw.sin(), step * pitch.sin(), xz_length * yaw.cos());

        for key in pressed.iter() {
            match key {
                glutin::event::VirtualKeyCode::Left => camera.rotate(0.0, -rstep, 0.0),
                glutin::event::VirtualKeyCode::Right => camera.rotate(0.0, rstep, 0.0),
                // glutin::event::VirtualKeyCode::C => y += step,
                // glutin::event::VirtualKeyCode::E => y -= step,
                glutin::event::VirtualKeyCode::W => camera.translate(movement),
                glutin::event::VirtualKeyCode::S => camera.translate(movement * -1.0),

                glutin::event::VirtualKeyCode::Down => camera.rotate(rstep, 0.0, 0.0),
                glutin::event::VirtualKeyCode::Up => camera.rotate(-rstep, 0.0, 0.0),
                // glutin::event::VirtualKeyCode::Right => ry += rstep,
                // glutin::event::VirtualKeyCode::Left => ry -= rstep,
                _ => ()
            }
        }

        engine.render(main_camera);
    });
}