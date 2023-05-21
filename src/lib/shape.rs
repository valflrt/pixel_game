use crate::vec2::Vec2;

#[derive(Debug, Clone)]
pub enum Shape {
    Rect(Vec2<i32>),
}

impl Shape {
    pub fn new_rect<D>(dims: D) -> Self
    where
        D: Into<Vec2<i32>>,
    {
        Shape::Rect(dims.into())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Boundaries {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
