mod vertex_array;

use vertex_array::*;

pub struct Model {
    shader: String,
    vertex_array: VertexArray
}

impl Model {
    pub fn new(points: &[f32], indices: &[u32], shader: String) -> Model {
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
}