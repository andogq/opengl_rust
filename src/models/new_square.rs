use cgmath::Vector3;

use crate::engine::traits::{ WorldPosition, Renderable };

pub struct NewSqaure {
    translation: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: Vector3<f32>
}

impl WorldPosition for NewSqaure {
    fn get_translation(&self) -> &cgmath::Vector3<f32> {
        &self.translation
    }

    fn get_rotation(&self) -> &cgmath::Vector3<f32> {
        &self.rotation
    }

    fn get_scale(&self) -> &cgmath::Vector3<f32> {
        &self.scale
    }
}

impl Renderable for NewSqaure {
    
}

impl NewSqaure {
    pub fn new() -> Box<NewSqaure> {
        Box::new(NewSqaure {
            translation: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(0.0, 0.0, 0.0)
        })
    }
}