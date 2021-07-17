use cgmath::{ Vector3, Matrix4 };

pub trait WorldPosition {
    fn get_translation(&self) -> &Vector3<f32>;
    fn get_rotation(&self) -> &Vector3<f32>;
    fn get_scale(&self) -> f32;

    fn get_model_matrix(&self) -> &Matrix4<f32>;

    fn translate(&mut self, translation: Vector3<f32>);
    fn rotate(&mut self, rotation: Vector3<f32>);
    fn scale(&mut self, scale: f32);
}