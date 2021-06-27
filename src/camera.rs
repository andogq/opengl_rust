use cgmath::Matrix4;

pub struct Camera {
    pub projection_matrix: Matrix4<f32>,
    pub view_matrix: Matrix4<f32>
}