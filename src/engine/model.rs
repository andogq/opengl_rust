mod vertex_array;

use vertex_array::{VertexArray, Layout};

use super::object::Object;

pub struct Model {
    shader: usize,
    vertex_array: VertexArray,

    objects: Vec<Object>
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
            vertex_array,

            objects: Vec::new()
        }
    }

    pub fn get_shader(&self) -> usize {
        self.shader
    }

    pub fn get_vertex_array(&self) -> &VertexArray {
        &self.vertex_array
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn get_objects(&self) -> &Vec<Object> {
        &self.objects
    }
}