use cgmath::Vector3;

pub struct Object {
    model: String,
    position: Vector3<f32>,
    scale: Vector3<f32>
}

impl Object {
    pub fn new(model: &str, position: Vector3<f32>, scale: Vector3<f32>) -> Object {
        Object{ 
            model: String::from(model),
            position,
            scale
        }
    }
}