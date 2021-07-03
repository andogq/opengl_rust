mod vertex_array;

use vertex_array::{VertexArray, Layout};

pub struct Model {
    shader: usize,
    vertex_array: VertexArray
}

impl Model {
    pub fn new(points: &[f32], indices: &[u32], shader: usize) -> Model {
        let mut vertex_array = VertexArray::new(Vec::from([
            Layout {
                normalised: gl::FALSE,
                size: 3
            }
        ]));

        vertex_array.set_data(points);
        vertex_array.set_indices(indices);

        Model {
            shader,
            vertex_array
        }
    }

    pub fn get_shader(&self) -> usize {
        self.shader
    }

    pub fn get_vertex_array(&self) -> &VertexArray {
        &self.vertex_array
    }
}