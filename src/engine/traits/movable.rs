use super::{ WorldPosition };

use cgmath::Vector3;

pub trait Movable: WorldPosition {
    fn translate(&mut self, translation: Vector3<f32>);
    fn rotate(&mut self, rotation: Vector3<f32>);
    fn scale(&mut self, scale: f32);
}