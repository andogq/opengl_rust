use cgmath;

pub trait WorldPosition {
    fn get_translation(&self) -> &cgmath::Vector3<f32>;
    fn get_rotation(&self) -> &cgmath::Vector3<f32>;
    fn get_scale(&self) -> &cgmath::Vector3<f32>;
}