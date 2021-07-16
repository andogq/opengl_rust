use cgmath::{ Vector3, Matrix4, Rad };
use super::super::{ VertexArray, Shader, Layout, traits };

pub struct Square {
    translation: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: f32,

    model_matrix: Matrix4<f32>,

    vertex_array: VertexArray,
    shader: Shader
}

impl Square {
    pub fn new() -> Square {
        let mut vertex_array = VertexArray::new(Vec::from([
            Layout {
                normalised: gl::FALSE,
                size: 3
            }
        ]));

        let points: [f32; 12] = [
            0.0, 0.0, 0.0,
            1.0, 0.0, 0.0,
            1.0, 1.0, 0.0,
            0.0, 1.0, 0.0
        ];
        let indices = [
            0, 1, 2,
            2, 3, 0
        ];

        let translation = Vector3::new(0.0, 0.0, -10.0);
        let rotation = Vector3::new(0.0, 0.0, 0.0);
        let scale = 1.0;

        vertex_array.set_data(&points);
        vertex_array.set_indices(&indices);

        let rotation_matrix = Matrix4::from_angle_x(Rad(rotation.x)) * Matrix4::from_angle_y(Rad(rotation.x)) * Matrix4::from_angle_z(Rad(rotation.x));
        let model_matrix = Matrix4::from_translation(translation) * rotation_matrix * Matrix4::from_scale(scale);

        Square {
            translation,
            rotation,
            scale,

            model_matrix,

            vertex_array,
            shader: Shader::new("red", false)
        }
    }
}

impl traits::WorldPosition for Square {
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
}

impl traits::Renderable for Square {
    fn get_vertex_array(&self) -> &VertexArray {
        &self.vertex_array
    }

    fn get_shader(&self) -> &Shader {
        &self.shader
    }
}