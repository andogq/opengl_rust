use gl::types::*;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder};

use cgmath::{Matrix4, SquareMatrix, Vector3, ortho};

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::mem;

use std::fs::read_to_string;
use std::time;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

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
    let positions: [GLfloat; 8] = [
        0.0, 1.0,
        1.0, 1.0,
        1.0, 0.0,
        0.0, 0.0
    ];

    let indexes: [GLuint; 6] = [
        0, 1, 2,
        2, 3, 0
    ];
    
    let mut vertex_buffer: u32 = 0;
    let mut index_buffer: u32 = 0;
    let mut vertex_array: u32 = 0;

    let program = Program::new("basic");
    program.bind();
    
    unsafe {
        // Initialise vertex buffer
        gl::GenBuffers(1, &mut vertex_buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
        gl::BufferData(gl::ARRAY_BUFFER, (positions.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, mem::transmute(&positions[0]), gl::STATIC_DRAW);

        // Create a vertex array
        gl::GenVertexArrays(1, &mut vertex_array);
        gl::BindVertexArray(vertex_array);

        // Create an attribute
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE as GLboolean, 0, std::ptr::null());
        
        // Initialise index buffer
        gl::GenBuffers(1, &mut index_buffer);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indexes.len() * mem::size_of::<i32>()) as isize, mem::transmute(&indexes[0]), gl::STATIC_DRAW);
        
        // Ensure there's no errors
        check_errors();
    };

    let projection = ortho(0.0, WINDOW_WIDTH as f32, 0.0, WINDOW_HEIGHT as f32, 0.0, 1.0);
    let view = Matrix4::identity();
    let mut model = Matrix4::from_scale(100.0);
    
    let u_mvp_matrix = program.get_uniform( "u_mvp_matrix");

    let u_color = program.get_uniform("u_color");

    let mut r = 0.0;
    let mut b = 1.0;
    let mut g = 0.0;

    let mut dr = 0.0001;
    let mut dg = -0.0001;
    let mut db = 0.00005;
    
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

                unsafe { gl::Uniform4f(u_color, r, g, b, 1.0) };

                model = Matrix4::from_translation(Vector3::new(1.0, 0.0, 0.0)) * model;

                let mvp_matrix: [[f32; 4]; 4] = (projection * view * model).into();
                unsafe { gl::UniformMatrix4fv(u_mvp_matrix, 1, gl::FALSE, mvp_matrix[0].as_ptr()) };

                unsafe {
                    gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
                    gl::BindVertexArray(vertex_array);

                    gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
                };

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

struct Program {
    id: u32
}

impl Program {
    fn new(name: &str) -> Program {
        // Load shaders from their respective files
        let path = format!("./res/shaders/{}", name);

        let vertex_shader_source = read_to_string(format!("{}/vertex.glsl", path)).expect("Problem reading shader");
        let fragment_shader_source = read_to_string(format!("{}/fragment.glsl", path)).expect("Problem reading shader");

        let vertex_shader = Shader::new(ShaderType::Vertex, &vertex_shader_source);
        let fragment_shader = Shader::new(ShaderType::Fragment, &fragment_shader_source);

        let id = unsafe { gl::CreateProgram() };

        unsafe {
            // Attach the shaders
            gl::AttachShader(id, vertex_shader.id);
            gl::AttachShader(id, fragment_shader.id);

            // Link and check the program
            gl::LinkProgram(id);
            gl::ValidateProgram(id);
        }

        unsafe {
            // Should be done when they go out of scope
            gl::DeleteShader(vertex_shader.id);
            gl::DeleteShader(fragment_shader.id);
        }

        Program {
            id
        }
    }

    fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        };
    }

    fn get_uniform(&self, uniform_name: &str) -> i32 {
        // Bind the CString to a variable so it doesn't go out of scope
        let uniform_name_cstring = CString::new(uniform_name).expect("Invalid string to be converted to CString (might have null byte)");

        // Able to use the pointer here because it hasn't been freed, and return the location
        let location = unsafe { gl::GetUniformLocation(self.id, uniform_name_cstring.as_ptr()) };

        if location == -1 {
            panic!("Uniform {} doesn't exist", uniform_name);
        }

        location
    }
}

enum ShaderType {
    Vertex,
    Fragment
}

struct Shader {
    id: u32
}

impl Shader {
    fn new(shader_type: ShaderType, source: &String) -> Shader {
        // Create the shader
        let id = unsafe { gl::CreateShader(match shader_type { ShaderType::Vertex => gl::VERTEX_SHADER, ShaderType::Fragment => gl::FRAGMENT_SHADER }) };

        // Load the source and compile the shader
        unsafe {
            gl::ShaderSource(id, 1, &(source.as_ptr() as *const c_char), std::ptr::null());
            gl::CompileShader(id);
        };

        // Check for compilation errors
        let mut compilation_result = 0;
        unsafe { gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut compilation_result) };
        if compilation_result == gl::FALSE as i32 {
            println!("Problem compiling shader");

            // Get the size of the error to create the buffer
            let mut error_length = 0;
            unsafe { gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut error_length) };

            // Work out how to generate character buffer to pass to shader to get error
            let mut buffer: Vec<u8> = Vec::with_capacity((error_length + 1) as usize);
            buffer.extend([b' '].iter().cycle().take(error_length as usize));

            let error = unsafe { CString::from_vec_unchecked(buffer) };
            unsafe{ gl::GetShaderInfoLog(id, error_length, std::ptr::null_mut(), error.as_ptr() as *mut i8) };

            println!("{:?}", error);
        };

        // Return the shader struct
        Shader {
            id
        }
    }
}