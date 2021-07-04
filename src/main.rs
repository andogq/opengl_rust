use gl::types::*;

use cgmath::Vector3;

mod window;
use window::Window;

mod engine;
use engine::Engine;

mod logger;
use logger::{ Logger, Level };

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

const PI : f32 = 3.141592653589793;

fn main() {
    let mut log = Logger::new(Level::Debug);

    log.debug("test debug message");
    log.info("test info message");
    log.warn("test warn message");
    log.error("test error message");

    log.set_level(Level::Normal);

    log.debug("test debug message");
    log.info("test info message");
    log.warn("test warn message");
    log.error("test error message");

    log.set_level(Level::Quiet);

    log.debug("test debug message");
    log.info("test info message");
    log.warn("test warn message");
    log.error("test error message");

    log.set_level(Level::Mute);

    log.debug("test debug message");
    log.info("test info message");
    log.warn("test warn message");
    log.error("test error message");

    // Set up the positions
    let positions: [GLfloat; 12] = [
        -1.0,  1.0,  0.0,
         1.0,  1.0,  0.0,
         1.0, -1.0,  0.0,
        -1.0, -1.0,  0.0
    ];

    let indices: [GLuint; 6] = [
        0, 1, 2,
        2, 3, 0
    ];
    
    let cube = [
        -1.0, -1.0, -1.0, // 0
         1.0, -1.0, -1.0, // 1
         1.0, -1.0,  1.0, // 2
        -1.0, -1.0,  1.0, // 3
        -1.0,  1.0, -1.0, // 4
         1.0,  1.0, -1.0, // 5
         1.0,  1.0,  1.0, // 6
        -1.0,  1.0,  1.0  // 7
    ];
    
    let cube_indices = [
        // Top face
        7, 6, 5,
        5, 4, 7,

        // Bottom face
        0, 1, 2,
        2, 3, 0,

        // Front face
        4, 1, 0,
        4, 5, 1,

        // Back face
        2, 6, 7,
        7, 3, 2,

        // Left face
        7, 4, 0,
        0, 3, 7,

        // Right face
        5, 2, 1,
        2, 5, 6,
    ];

    /*
        NEW MAIN
    */
    let window = Window::new();
    
    let mut engine = Engine::new();

    engine.init();

    let lighting_shader = engine.add_shader("lighting", true);
    let cube_model = engine.add_model(&cube, &cube_indices, lighting_shader);
    engine.add_object(cube_model, Vector3::new(0.0, 0.0, 0.0), Vector3::new(100.0, 100.0, 100.0));
    
    let red_shader = engine.add_shader("red", false);
    let red_cube_model = engine.add_model(&cube, &cube_indices, red_shader);
    // let square_model = engine.add_model(&positions, &indices, red_shader);
    engine.add_object(red_cube_model, Vector3::new(0.0, 500.0, -500.0), Vector3::new(5.0, 5.0, 5.0));

    let main_camera = engine.add_camera(Vector3::new(0.0, 0.0, -100.0), Vector3::new(0.0, 0.0, 0.0), (WINDOW_WIDTH as f32)/(WINDOW_HEIGHT as f32), PI/2.0);

    let step = 10.0;
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