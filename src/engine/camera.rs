use cgmath::{Vector3, perspective, Matrix4, Rad};

use super::traits::{ WorldPosition };

pub struct Camera {
    translation: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: Vector3<f32>,
    aspect: f32,
    fov: f32
}

impl Camera {
    pub fn new(translation: Vector3<f32>, rotation: Vector3<f32>, aspect: f32, fov: f32) -> Camera {
        Camera {
            translation,
            rotation,
            scale: Vector3::new(1.0, 1.0, 1.0),
            aspect,
            fov
        }
    }

    pub fn get_projection_matrix(&self) -> Matrix4<f32> {
        perspective(Rad(self.fov), self.aspect, 0.1, 10000.0)
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_angle_x(Rad(self.rotation.x)) * Matrix4::from_angle_y(Rad(self.rotation.y)) * Matrix4::from_angle_z(Rad(self.rotation.z)) * Matrix4::from_translation(self.translation)

    }

    pub fn translate(&mut self, difference: Vector3<f32>) {
        self.translation += difference;
    }

    pub fn rotate(&mut self, x: f32, y: f32, z: f32) {
        self.rotation.x += x;
        self.rotation.y += y;
        self.rotation.z += z;
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