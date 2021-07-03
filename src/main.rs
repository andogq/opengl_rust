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
        4, 5, 6,
        6, 7, 4,

        // Bottom face
        0, 1, 2,
        2, 3, 0,

        // Front face
        0, 1, 4,
        4, 5, 1,

        // Back face
        2, 6, 7,
        7, 3, 2,

        // Left face
        0, 4, 7,
        7, 3, 0,

        // Right face
        1, 2, 5,
        2, 5, 6,
    ];

    /*
        NEW MAIN
    */
    let window = Window::new();
    
    let mut engine = Engine::new();

    engine.init();

    let basic_shader = engine.add_shader("basic");
    // let square_model = engine.add_model(&positions, &indices, basic_shader);

    let cube_model = engine.add_model(&cube, &cube_indices, basic_shader);

    engine.add_object(cube_model, Vector3::new(0.0, 0.0, 0.0), Vector3::new(100.0, 100.0, 1.0));
    // engine.add_object(square_model, Vector3::new(500.0, 0.0, 0.0), Vector3::new(100.0, 100.0, 1.0));

    let main_camera = engine.add_camera(Vector3::new(500.0, -500.0, -1000.0), Vector3::new(PI/4.0, PI/4.0, 0.0), (WINDOW_WIDTH as f32)/(WINDOW_HEIGHT as f32), PI/2.0);

    unsafe {
        gl::Disable(gl::CULL_FACE);
    }

    window.run(move || {
        engine.get_camera(main_camera).rotate(0.0, 0.0, 0.0);

        engine.render(main_camera);
    });
}