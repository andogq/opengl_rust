use super::{ WorldPosition };

pub trait Movable: WorldPosition {
    fn translate(&mut self);
    fn rotate(&mut self);
    fn scale(&mut self);
}