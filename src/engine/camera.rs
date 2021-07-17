use super::{ WorldPosition };

use cgmath::{ Vector3, Matrix4, Rad, perspective };

pub struct Camera {
    translation: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: f32,
    
    model_matrix: Matrix4<f32>,
    projection_matrix: Matrix4<f32>
}

impl Camera {
    pub fn new(fov: f32, aspect: f32, near: f32, far: f32) -> Camera {
        Camera {
            translation: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: 1.0,
            model_matrix: Matrix4::from_scale(1.0),
            projection_matrix: perspective(Rad(fov), aspect, near, far)
        }
    }

    fn update_model_matrix(&mut self) {
        let rotation_matrix = Matrix4::from_angle_x(Rad(self.rotation.x)) * Matrix4::from_angle_y(Rad(self.rotation.x)) * Matrix4::from_angle_z(Rad(self.rotation.x));
        self.model_matrix = Matrix4::from_translation(self.translation) * rotation_matrix * Matrix4::from_scale(self.scale);
    }

    pub fn get_projection_matrix(&self) -> &Matrix4<f32> {
        &self.projection_matrix
    }
}

impl WorldPosition for Camera {
    fn get_translation(&self) -> &Vector3<f32> {
        &self.translation
    }

    fn get_rotation(&self) -> &Vector3<f32> {
        &self.rotation
    }

    fn get_scale(&self) -> f32 {
        self.scale
    }

    fn get_model_matrix(&self) -> &Matrix4<f32> {
        &self.model_matrix
    }

    fn translate(&mut self, translation: Vector3<f32>) {
        self.translation += translation;
        self.update_model_matrix();
    }

    fn rotate(&mut self, rotation: Vector3<f32>) {
        self.rotation += rotation;
        self.update_model_matrix();
    }

    fn scale(&mut self, scale: f32) {
        self.scale += scale;
        self.update_model_matrix();
    }
}