// use crate::camera::*;

// pub struct Renderer {
//     camera: Camera,
//     objects: Vec<Object>
// }

// impl Renderer {
//     pub fn draw(&self) {
//         let vp_matrix = self.camera.projection_matrix * self.camera.view_matrix;
//         for object in self.objects.iter() {
//             // Create the mvp matrix
//             let mvp_matrix = vp_matrix * object.model_matrix;

//             // Bind the program
//             object.program.bind();

//             // Load it into the program
//             object.program.set_uniform("u_mvp_matrix", &mvp_matrix);

//             // vertex_array.add()
//         }
//     }
// }