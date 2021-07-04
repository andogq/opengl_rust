use gl::types::*;

use cgmath::Vector3;

mod window;
use window::Window;

mod engine;
use engine::Engine;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

const PI : f32 = 3.141592653589793;

fn main() {
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

    unsafe {
        gl::Enable(gl::CULL_FACE);
        // gl::Disable(gl::BLEND);
    }

    let step = 10.0;
    let rstep = 0.01;

    window.run(move |pressed| {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;


        let mut rx = 0.0;
        let mut ry = 0.0;
        let mut rz = 0.0;

        for key in pressed.iter() {
            match key {
                glutin::event::VirtualKeyCode::A => x += step,
                glutin::event::VirtualKeyCode::D => x -= step,
                glutin::event::VirtualKeyCode::C => y += step,
                glutin::event::VirtualKeyCode::E => y -= step,
                glutin::event::VirtualKeyCode::W => z += step,
                glutin::event::VirtualKeyCode::S => z -= step,


                glutin::event::VirtualKeyCode::Down => rx += rstep,
                glutin::event::VirtualKeyCode::Up => rx -= rstep,
                glutin::event::VirtualKeyCode::Right => ry += rstep,
                glutin::event::VirtualKeyCode::Left => ry -= rstep,
                _ => ()
            }
        }

        engine.get_camera(main_camera).translate(x, y, z);
        engine.get_camera(main_camera).rotate(rx, ry, rz);

        engine.render(main_camera);
    });
}