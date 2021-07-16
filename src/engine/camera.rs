use super::{ WorldPosition };

use cgmath::Vector3;

pub struct Camera {
    translation: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: Vector3<f32>
}

impl Camera {
    fn new() -> Camera {
        Camera {
            translation: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0)
        }
    }
}

impl WorldPosition for Camera {
    fn get_translation(&self) -> &Vector3<f32> {
        &self.translation
    }

    fn get_rotation(&self) -> &Vector3<f32> {
        &self.rotation
    }

    fn get_scale(&self) -> &Vector3<f32> {
        &self.scale
    }
}