use gl::types::*;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, PossiblyCurrent};

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::mem;

fn main() {
    // Create the window and event loop
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("Hello");

    // Load the context
    let context = ContextBuilder::new().build_windowed(wb, &el).unwrap();
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
        -0.5,  0.5,
         0.5,  0.5,
         0.5, -0.5,
        -0.5, -0.5
    ];

    let indexes: [GLuint; 6] = [
        0, 1, 2,
        2, 3, 0
    ];

    let vertex_shader_source = String::from(r#"
        #version 330 core
        layout(location = 0) in vec4 position;

        void main() {
            gl_Position = position;
        }"#);
    let fragment_shader_source = String::from(r#"
        #version 330 core
        layout(location = 0) out vec4 color;
        
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }"#);

    let program;
    
    let vertex_shader = Shader::new(ShaderType::Vertex, &vertex_shader_source);
    let fragment_shader = Shader::new(ShaderType::Fragment, &fragment_shader_source);
    
    let mut vertex_buffer: u32 = 0;
    let mut index_buffer: u32 = 0;
    let mut vertex_array: u32 = 0;

    unsafe {
        program = gl::CreateProgram();

        gl::AttachShader(program, vertex_shader.id);
        gl::AttachShader(program, fragment_shader.id);
        gl::LinkProgram(program);
        gl::ValidateProgram(program);

        gl::DeleteShader(vertex_shader.id);
        gl::DeleteShader(fragment_shader.id);

        gl::UseProgram(program);
    };
    
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
    
    println!("[+] Beginning main loop");

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
                draw(&context, &vertex_buffer, &vertex_array, &program);
            },
            _ => (),
        }
    });
}

fn draw(context: &glutin::ContextWrapper<PossiblyCurrent, glutin::window::Window>, vertex_buffer: &u32, vertex_array: &u32, program: &u32) {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
        
        gl::UseProgram(*program);

        gl::BindBuffer(gl::ARRAY_BUFFER, *vertex_buffer);
        gl::BindVertexArray(*vertex_array);

        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
    };

    context.swap_buffers().unwrap();

    check_errors();
}

fn check_errors() {
    loop {
        let error = unsafe { gl::GetError() };

        if error != 0 { println!("[!] OpenGL Error: 0x{:x}", error); }
        else { break; }
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