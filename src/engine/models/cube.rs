use cgmath::{ Vector3, Matrix4, Rad };
use super::super::{ VertexArray, Shader, Layout, traits };

pub struct Cube {
    translation: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: f32,

    model_matrix: Matrix4<f32>,

    vertex_array: VertexArray,
    shader: Shader
}

impl Cube {
    pub fn new() -> Cube {
        let mut vertex_array = VertexArray::new(Vec::from([
            Layout {
                normalised: gl::FALSE,
                size: 3
            }
        ]));

        let points: [f32; 24] = [
            -1.0, -1.0, -1.0, // 0
             1.0, -1.0, -1.0, // 1
             1.0, -1.0,  1.0, // 2
            -1.0, -1.0,  1.0, // 3
            -1.0,  1.0, -1.0, // 4
             1.0,  1.0, -1.0, // 5
             1.0,  1.0,  1.0, // 6
            -1.0,  1.0,  1.0  // 7
        ];
        let indices = [
            // Top face
            7, 6, 5,
            5, 4, 7,

            // Bottom face
            0, 1, 2,
            2, 3, 0,

            // Front face
            4, 1, 0,
            4, 5, 1,

            // Back face
            2, 6, 7,
            7, 3, 2,

            // Left face
            7, 4, 0,
            0, 3, 7,

            // Right face
            5, 2, 1,
            2, 5, 6,
        ];

        let translation = Vector3::new(5.0, 0.0, -10.0);
        let rotation = Vector3::new(0.0, 0.0, 0.0);
        let scale = 1.0;

        vertex_array.set_data(&points);
        vertex_array.set_indices(&indices);

        let rotation_matrix = Matrix4::from_angle_x(Rad(rotation.x)) * Matrix4::from_angle_y(Rad(rotation.x)) * Matrix4::from_angle_z(Rad(rotation.x));
        let model_matrix = Matrix4::from_translation(translation) * rotation_matrix * Matrix4::from_scale(scale);

        Cube {
            translation,
            rotation,
            scale,

            model_matrix,

            vertex_array,
            shader: Shader::new("lighting", true)
        }
    }

    fn update_model_matrix(&mut self) {
        let rotation_matrix = Matrix4::from_angle_x(Rad(self.rotation.x)) * Matrix4::from_angle_y(Rad(self.rotation.x)) * Matrix4::from_angle_z(Rad(self.rotation.x));
        self.model_matrix = Matrix4::from_translation(self.translation) * rotation_matrix * Matrix4::from_scale(self.scale);
    }
}

impl traits::WorldPosition for Cube {
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

impl traits::Renderable for Cube {
    fn get_vertex_array(&self) -> &VertexArray {
        &self.vertex_array
    }

    fn get_shader(&self) -> &Shader {
        &self.shader
    }
}