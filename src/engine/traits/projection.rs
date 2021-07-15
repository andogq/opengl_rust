use cgmath::Matrix4;

pub trait Projection {
    fn get_projection(&self) -> Matrix4<f32>;
}