use gl::types::*;

use cgmath::{Matrix4, Vector3, Rad, perspective};

use std::ffi::{CStr};

use std::time;

mod window;
use window::Window;

mod engine;
use engine::Engine;

mod renderer;
mod camera;

// use object::program::*;
// use object::vertex_array::*;

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

    /*
        NEW MAIN
    */
    let mut window = Window::new();
    
    let mut engine = Engine::new();

    {
        engine.init();
    
        let basic_shader = engine.add_shader("basic");
        let square_model = engine.add_model(&positions, &indices, basic_shader);

        engine.add_object(square_model, Vector3::new(0.0, 0.0, 0.0), Vector3::new(100.0, 100.0, 1.0));
        engine.add_object(square_model, Vector3::new(500.0, 0.0, 0.0), Vector3::new(100.0, 100.0, 1.0));
    
        engine.add_camera("main", Vector3::new(0.0, 0.0, -1000.0), Vector3::new(0.0, 0.0, 0.0), (WINDOW_WIDTH as f32)/(WINDOW_HEIGHT as f32), PI/2.0);
    }

    window.run(move || {
        engine.render("main");
    });
    
    // let program = Program::new("basic");
    // program.bind();

    // let mut vertex_array = VertexArray::new( Vec::from([
        // Layout {
        //     normalised: gl::FALSE,
        //     size: 3
        // }
    // ]));

    // vertex_array.set_data(&positions);
    // vertex_array.set_indices(&indices);
    
    // let projection = ortho(-(WINDOW_WIDTH as f32)/2.0, (WINDOW_WIDTH as f32)/2.0, -(WINDOW_HEIGHT as f32)/2.0, (WINDOW_HEIGHT as f32)/2.0, -10000.0, 10000.0);
    let projection = perspective(cgmath::Rad(50.0/180.0*PI), (WINDOW_WIDTH as f32)/(WINDOW_HEIGHT as f32), 0.1, 10000.0);
    let model = Matrix4::from_scale(1.0);
    let mut camera_position = Vector3::new(0.0, 0.0, -1.0);
    let mut camera_rotation = Vector3::new(0.0, 0.0, 0.0);
    
    // let u_mvp_matrix = program.get_uniform( "u_mvp_matrix");

    let mut last_draw: time::Instant = time::Instant::now();

    // Run the event loop
    // el.run(move |event, _, control_flow| {
    //     // Check what type of event has been called
    //     match event {
    //         Event::LoopDestroyed => return,
    //         Event::WindowEvent { event, .. } => match event {
    //             WindowEvent::Resized(physical_size) => context.resize(physical_size),
    //             WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
    //             WindowEvent::KeyboardInput {
    //                 input: glutin::event::KeyboardInput {
    //                     virtual_keycode: Some(virtual_code),
    //                     state,
    //                     ..
    //                 },
    //                 ..
    //             } => match (virtual_code, state) {
    //                 (glutin::event::VirtualKeyCode::A, glutin::event::ElementState::Pressed) => camera_position.x += 0.5,
    //                 (glutin::event::VirtualKeyCode::D, glutin::event::ElementState::Pressed) => camera_position.x -= 0.5,
    //                 (glutin::event::VirtualKeyCode::W, glutin::event::ElementState::Pressed) => camera_position.y -= 0.5,
    //                 (glutin::event::VirtualKeyCode::S, glutin::event::ElementState::Pressed) => camera_position.y += 0.5,
    //                 (glutin::event::VirtualKeyCode::Up, glutin::event::ElementState::Pressed) => camera_position.z += 0.5,
    //                 (glutin::event::VirtualKeyCode::Down, glutin::event::ElementState::Pressed) => camera_position.z -= 0.5,
    //                 _ => ()
    //             },
    //             _ => ()
    //         },
    //         Event::RedrawRequested(_) => {
    //             unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };

    //             // program.bind();

    //             // unsafe { gl::Uniform4f(u_color, r, g, b, 1.0) };

    //             let view = Matrix4::from_angle_x(cgmath::Rad(camera_rotation.x)) * Matrix4::from_angle_y(cgmath::Rad(camera_rotation.y)) * Matrix4::from_angle_z(cgmath::Rad(camera_rotation.z)) * Matrix4::from_translation(camera_position);
    //             let mvp_matrix: [[f32; 4]; 4] = (projection * view * model).into();
    //             unsafe { gl::UniformMatrix4fv(u_mvp_matrix, 1, gl::FALSE, mvp_matrix[0].as_ptr()) };

    //             // vertex_array.bind();

    //             unsafe { gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null()) };

    //             context.swap_buffers().unwrap();

    //             check_errors();
                
    //             last_draw = time::Instant::now();
    //         },
    //         _ => (),
    //     }

    //     match *control_flow {
    //         // Ensure to actually exit
    //         ControlFlow::Exit => (),
    //         _ => {
    //             // Calculate when next frame should be drawn, and trigger a draw call or wait
    //             let next_draw = last_draw + time::Duration::from_millis(1000 / FPS as u64);
        
    //             if next_draw <= time::Instant::now() { context.window().request_redraw(); }
    //             else { *control_flow = ControlFlow::WaitUntil(next_draw); }
    //         }
    //     }
    // });
}