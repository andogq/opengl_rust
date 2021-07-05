mod vertex_array;

use vertex_array::{VertexArray, Layout};

use super::object::Object;

pub enum RenderMode {
    Points,
    Triangles,
    TriangleStrip
}

pub struct Model {
    shader: usize,
    vertex_array: VertexArray,
    render_mode: RenderMode,

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
            render_mode: RenderMode::Triangles,

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
    
    pub fn set_render_mode(&mut self, render_mode: RenderMode) {
        self.render_mode = render_mode;
    }

    pub fn get_render_mode(&self) -> &RenderMode {
        &self.render_mode
    }
}