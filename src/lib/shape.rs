use crate::vec2::Vec2;

#[derive(Debug, Clone)]
pub enum Shape {
    Rect(Vec2),
}

impl Shape {
    pub fn new_rect(dims: Vec2) -> Self {
        Shape::Rect(dims.into())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Boundaries {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}
