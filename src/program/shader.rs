use std::os::raw::c_char;
use std::ffi::{CString};

pub enum ShaderType {
    Vertex,
    Fragment
}

pub struct Shader {
    pub id: u32
}

impl Shader {
    pub fn new(shader_type: ShaderType, source: &String) -> Shader {
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