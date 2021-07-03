use cgmath::{Vector3, Matrix4};

pub struct Object {
    model: usize,
    position: Vector3<f32>,
    scale: Vector3<f32>
}

impl Object {
    pub fn new(model: usize, position: Vector3<f32>, scale: Vector3<f32>) -> Object {
        Object{ 
            model,
            position,
            scale
        }
    }

    pub fn get_model(&self) -> usize {
        self.model
    }

    pub fn model_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.position) * Matrix4::from_scale(self.scale.x)
    }
}