use cgmath::{Vector3, perspective, Matrix4, Rad};

pub struct Camera {
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    aspect: f32,
    fov: f32
}

impl Camera {
    pub fn new(position: Vector3<f32>, rotation: Vector3<f32>, aspect: f32, fov: f32) -> Camera {
        Camera {
            position,
            rotation,
            aspect,
            fov
        }
    }

    pub fn vp_matrix(&self) -> Matrix4<f32> {
        let projection = perspective(Rad(self.fov), self.aspect, 0.1, 10000.0);
        let view = Matrix4::from_angle_x(Rad(self.rotation.x)) * Matrix4::from_angle_y(Rad(self.rotation.y)) * Matrix4::from_angle_z(Rad(self.rotation.z)) * Matrix4::from_translation(self.position);
        projection * view
    }
}