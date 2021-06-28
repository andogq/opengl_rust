mod vertex_array;

use vertex_array::*;

pub struct Model {
    id: u32,
    shader: u32,
    vertex_array: VertexArray
}

impl Model {
    pub fn new(points: &[f32], shader: u32) -> Model {
        Model {
            id: 0,
            shader: 0,
            vertex_array: VertexArray::new(Vec::new())
        }
    }

    pub fn set_points(&mut self) {
        
    }
}