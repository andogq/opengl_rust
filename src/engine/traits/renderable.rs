use super::WorldPosition;
use super::super::{ VertexArray, Shader };

pub trait Renderable: WorldPosition {
    fn get_vertex_array(&self) -> &VertexArray;
    fn get_shader(&self) -> &Shader;
}