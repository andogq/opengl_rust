use super::{Camera, Light, Renderable, traits::WorldPosition};

pub struct Renderer {

}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {

        }
    }

    pub fn render(&self, camera: &Camera, objects: &[Box<dyn Renderable>], lights: &[Box<Light>]) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);
        }

        for object in objects.iter() {
            let shader = object.get_shader();
            let vertex_array = object.get_vertex_array();
            
            shader.bind();
            vertex_array.bind();

            shader.set_uniform("u_model_matrix", object.get_model_matrix());
            shader.set_uniform("u_view_matrix", camera.get_model_matrix());
            shader.set_uniform("u_projection_matrix", camera.get_projection_matrix());

            unsafe { gl::DrawElements(gl::TRIANGLES, vertex_array.get_index_length(), gl::UNSIGNED_INT, std::ptr::null()) }
        }
    }
}