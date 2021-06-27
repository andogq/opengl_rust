use std::ffi::c_void;

use gl;

pub struct Layout {
    pub size: u32,
    pub normalised: gl::types::GLboolean
}

pub struct VertexArray {
    layout: Vec<Layout>,

    vertex_array: u32,
    vertex_buffer: u32,
    index_buffer: u32
}

impl VertexArray {
    pub fn new(layout: Vec<Layout>) -> VertexArray {
        // Generate the vertex array
        let mut vertex_array: u32 = 0;
        unsafe { gl::GenVertexArrays(1, &mut vertex_array) };

        // Generate vertex buffer
        let mut vertex_buffer: u32 = 0;
        unsafe { gl::GenBuffers(1, &mut vertex_buffer) };

        // Generate index buffer
        let mut index_buffer: u32 = 0;
        unsafe { gl::GenBuffers(1, &mut index_buffer) };

        let va = VertexArray {
            layout,

            vertex_array,
            vertex_buffer,
            index_buffer
        };

        let data_type = gl::FLOAT;

        // Set all the attributes
        va.bind();
        for (i, l) in va.layout.iter().enumerate() {
            unsafe {
                gl::EnableVertexAttribArray(i as u32);
                gl::VertexAttribPointer(i as u32, l.size as i32, data_type, l.normalised, 0, std::ptr::null());
            };
        }

        // Return the initialised va object
        va
    }

    pub fn set_data(&mut self, data: &[f32]) {
        self.bind();
        unsafe { gl::BufferData(gl::ARRAY_BUFFER, std::mem::size_of_val(data) as isize, data.as_ptr() as *const c_void, gl::STATIC_DRAW) };
    }

    pub fn set_indices(&mut self, indices: &[u32]) {
        self.bind();
        unsafe { gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, std::mem::size_of_val(indices) as isize, indices.as_ptr() as *const c_void, gl::STATIC_DRAW) };
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vertex_array);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer);
        }
    }
}