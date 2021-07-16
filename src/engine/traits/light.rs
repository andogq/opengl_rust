use super::{ WorldPosition };

use cgmath::Vector3;

pub trait Light: WorldPosition {
    fn get_color(&self) -> &Vector3<f32>;
    fn get_intensity(&self) -> f32;
}