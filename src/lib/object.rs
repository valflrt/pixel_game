use crate::{
    shape::{Boundaries, Shape},
    vec2::Vec2,
};

mod hitbox;

#[derive(Debug, Clone)]
pub struct Object {
    pos: Vec2,
    shape: Shape,
}

impl Object {
    pub fn new(pos: Vec2, shape: Shape) -> Self {
        Object {
            pos: pos.into(),
            shape,
        }
    }

    pub fn in_contact_with(&self, other: &Self) -> bool {
        let b1 = self.raw_boundaries();
        let b2 = other.raw_boundaries();

        b1.right >= b2.left && b1.left <= b2.right && b1.bottom >= b2.top && b1.top <= b2.bottom
    }

    pub fn in_contact_with_any(&self, others: &[&Self]) -> bool {
        let b1 = self.raw_boundaries();
        others.iter().any(|other| {
            let b2 = other.raw_boundaries();
            b1.right >= b2.left && b1.left <= b2.right && b1.bottom >= b2.top && b1.top <= b2.bottom
        })
    }

    pub fn raw_dims(&self) -> Vec2 {
        match &self.shape {
            Shape::Rect(dims) => *dims,
        }
    }

    pub fn raw_boundaries(&self) -> Boundaries {
        let dims = self.raw_dims();
        Boundaries {
            left: self.pos.0,
            top: self.pos.1,
            right: self.pos.0 + dims.0 + 1.,
            bottom: self.pos.1 + dims.1 + 1.,
        }
    }

    pub fn pos(&self) -> &Vec2 {
        &self.pos
    }
    pub fn pos_mut(&mut self) -> &mut Vec2 {
        &mut self.pos
    }

    pub fn shape(&self) -> &Shape {
        &self.shape
    }
}
