use crate::engine::model::Model;

pub fn new(corners: &[[[f32; 3]; 2]; 2], shader: usize) -> Model {
    let mut points: [f32; 12] = [0.0; 12];

    // Copy corners into points array
    let mut i = 0;
    for row in corners {
        for point in row {
            for coord in point {
                points[i] = coord.clone();
                i += 1;
            }
        }
    }

    // Indicies won't change
    let indices = [
        0, 1, 2,
        1, 2, 3
    ];

    Model::new(&points, &indices, shader)
}