use cgmath::Vector3;

pub trait WorldPosition {
    fn get_translation(&self) -> &Vector3<f32>;
    fn get_rotation(&self) -> &Vector3<f32>;
    fn get_scale(&self) -> &Vector3<f32>;
}