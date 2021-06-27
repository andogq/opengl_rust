use gl::types::*;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder};

use cgmath::{Matrix4, Vector3, ortho, perspective};

use std::ffi::{CStr};
use std::mem;

use std::time;

mod program;
use program::*;

mod renderer;
mod camera;
mod object;

use renderer::vertex_array::*;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

const PI : f32 = 3.141592653589793;

const FPS: u32 = 60;

fn main() {
    // Create the window and event loop
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("Hello").with_inner_size(glutin::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT));

    // Load the context
    let context = ContextBuilder::new().with_vsync(true).build_windowed(wb, &el).unwrap();
    let context = unsafe { context.make_current().unwrap() };

    // Load OpenGL function wrapper
    gl::load_with(|s| context.get_proc_address(s));

    // Get OpenGL version
    unsafe {
        let data = gl::GetString(gl::VERSION);
        println!("{}", String::from_utf8(CStr::from_ptr(data as *const i8).to_bytes().to_vec()).unwrap());
    };

    // Set the clear color
    unsafe { gl::ClearColor(0.0, 0.0, 0.0, 1.0) };

    // Set up the positions
    let positions: [GLfloat; 12] = [
        -1.0,  1.0,  0.0,
         1.0,  1.0,  0.0,
         1.0, -1.0,  0.0,
        -1.0, -1.0,  0.0
    ];

    let indexes: [GLuint; 6] = [
        0, 1, 2,
        2, 3, 0
    ];
    
    let mut index_buffer: u32 = 0;

    let program = Program::new("basic");
    program.bind();

    let mut vertex_array = VertexArray::new( Vec::from([
        Layout {
            normalised: gl::FALSE,
            size: 3
        }
    ]));

    vertex_array.set_data(&positions);
    
    unsafe {
        // Initialise index buffer
        gl::GenBuffers(1, &mut index_buffer);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indexes.len() * mem::size_of::<i32>()) as isize, mem::transmute(&indexes[0]), gl::STATIC_DRAW);
        
        // Ensure there's no errors
        check_errors();
    };

    // let projection = ortho(-(WINDOW_WIDTH as f32)/2.0, (WINDOW_WIDTH as f32)/2.0, -(WINDOW_HEIGHT as f32)/2.0, (WINDOW_HEIGHT as f32)/2.0, -10000.0, 10000.0);
    let projection = perspective(cgmath::Rad(50.0/180.0*PI), (WINDOW_WIDTH as f32)/(WINDOW_HEIGHT as f32), 0.1, 10000.0);
    let model = Matrix4::from_scale(1.0);
    let mut camera_position = Vector3::new(0.0, 0.0, -1.0);
    let mut camera_rotation = Vector3::new(0.0, 0.0, 0.0);
    
    let u_mvp_matrix = program.get_uniform( "u_mvp_matrix");

    let u_color = program.get_uniform("u_color");

    let mut r = 0.0;
    let mut b = 1.0;
    let mut g = 0.0;

    let mut dr = 0.001;
    let mut dg = -0.001;
    let mut db = 0.0005;
    
    println!("[+] Beginning main loop");

    let mut last_draw: time::Instant = time::Instant::now();

    // Run the event loop
    el.run(move |event, _, control_flow| {
        // Check what type of event has been called
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input: glutin::event::KeyboardInput {
                        virtual_keycode: Some(virtual_code),
                        state,
                        ..
                    },
                    ..
                } => match (virtual_code, state) {
                    (glutin::event::VirtualKeyCode::A, glutin::event::ElementState::Pressed) => camera_position.x += 0.5,
                    (glutin::event::VirtualKeyCode::D, glutin::event::ElementState::Pressed) => camera_position.x -= 0.5,
                    (glutin::event::VirtualKeyCode::W, glutin::event::ElementState::Pressed) => camera_position.y -= 0.5,
                    (glutin::event::VirtualKeyCode::S, glutin::event::ElementState::Pressed) => camera_position.y += 0.5,
                    (glutin::event::VirtualKeyCode::Up, glutin::event::ElementState::Pressed) => camera_position.z += 0.5,
                    (glutin::event::VirtualKeyCode::Down, glutin::event::ElementState::Pressed) => camera_position.z -= 0.5,
                    _ => ()
                },
                _ => ()
            },
            Event::RedrawRequested(_) => {
                unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };

                program.bind();

                r += dr;
                g += dg;
                b += db;

                if r <= 0.0 || r >= 1.0 { dr *= -1.0; }
                if g <= 0.0 || g >= 1.0 { dg *= -1.0; }
                if b <= 0.0 || b >= 1.0 { db *= -1.0; }

                // camera_rotation.x += PI/1000.0;
                // camera_rotation.y += PI/1000.0;

                unsafe { gl::Uniform4f(u_color, r, g, b, 1.0) };

                let view = Matrix4::from_angle_x(cgmath::Rad(camera_rotation.x)) * Matrix4::from_angle_y(cgmath::Rad(camera_rotation.y)) * Matrix4::from_angle_z(cgmath::Rad(camera_rotation.z)) * Matrix4::from_translation(camera_position);
                let mvp_matrix: [[f32; 4]; 4] = (projection * view * model).into();
                unsafe { gl::UniformMatrix4fv(u_mvp_matrix, 1, gl::FALSE, mvp_matrix[0].as_ptr()) };

                vertex_array.bind();

                unsafe { gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null()) };

                context.swap_buffers().unwrap();

                check_errors();
                
                last_draw = time::Instant::now();
            },
            _ => (),
        }

        match *control_flow {
            // Ensure to actually exit
            ControlFlow::Exit => (),
            _ => {
                // Calculate when next frame should be drawn, and trigger a draw call or wait
                let next_draw = last_draw + time::Duration::from_millis(1000 / FPS as u64);
        
                if next_draw <= time::Instant::now() { context.window().request_redraw(); }
                else { *control_flow = ControlFlow::WaitUntil(next_draw); }
            }
        }
    });
}

fn check_errors() {
    loop {
        let error = unsafe { gl::GetError() };

        if error != 0 { println!("[!] OpenGL Error: 0x{:x}", error); }
        else { break; }
    }
}