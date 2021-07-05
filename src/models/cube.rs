use crate::engine::model::Model;

pub fn new(shader: usize) -> Model {
    Model::new(
        &[
            -1.0, -1.0, -1.0, // 0
            1.0, -1.0, -1.0, // 1
            1.0, -1.0,  1.0, // 2
            -1.0, -1.0,  1.0, // 3
            -1.0,  1.0, -1.0, // 4
            1.0,  1.0, -1.0, // 5
            1.0,  1.0,  1.0, // 6
            -1.0,  1.0,  1.0  // 7
        ],
        &[
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
        ],
        shader
    )
}