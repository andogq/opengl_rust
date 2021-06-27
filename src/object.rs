use cgmath::Matrix4;

use crate::program::*;

pub struct Object {
    pub model_matrix: Matrix4<f32>,
    pub vertices: Vec<(f32, f32, f32)>,
    pub program: Program
}