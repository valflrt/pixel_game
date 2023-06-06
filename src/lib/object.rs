use crate::vec2::Vec2;

#[derive(Debug, Clone)]
pub struct Object {
    pos: Vec2,
    /// The dims used for collision and other stuff.
    dims: Vec2,
    image_offset: Vec2,
}

impl Object {
    pub fn new(pos: Vec2, dims: Vec2, image_offset: Option<Vec2>) -> Self {
        Object {
            pos,
            dims,
            image_offset: image_offset.unwrap_or(Vec2::ZERO),
        }
    }

    fn intersecting_hitboxes(a: Boundaries, b: Boundaries) -> bool {
        a.right >= b.left && a.left <= b.right && a.bottom >= b.top && a.top <= b.bottom
    }
    fn intersection_boundaries(a: Boundaries, b: Boundaries) -> Boundaries {
        Boundaries {
            left: a.left.max(b.left),
            right: a.right.min(b.right),
            top: a.top.max(b.top),
            bottom: a.bottom.min(b.bottom),
        }
    }

    /// Return true if the object is intersecting or in contact
    /// with the other object, return false otherwise.
    pub fn intersecting_with(&self, other: &Self) -> bool {
        Self::intersecting_hitboxes(self.boundaries(), other.boundaries())
    }

    /// Return the dimensions of the intersection of the object
    /// with another object.
    pub fn intersection_boundaries_with(&self, other: &Self) -> Boundaries {
        Self::intersection_boundaries(self.boundaries(), other.boundaries())
    }

    /// Return true if the object is intersecting or in contact
    /// with any of the other objects, return false otherwise.
    pub fn in_intersecting_with_any(&self, others: &[&Self]) -> bool {
        let b1 = self.boundaries();
        others.iter().any(|other| {
            let b2 = other.boundaries();
            Self::intersecting_hitboxes(b1, b2)
        })
    }

    pub fn pos(&self) -> &Vec2 {
        &self.pos
    }
    pub fn pos_mut(&mut self) -> &mut Vec2 {
        &mut self.pos
    }
    pub fn dims(&self) -> &Vec2 {
        &self.dims
    }
    pub fn image_pos(&self) -> Vec2 {
        self.pos + self.image_offset
    }

    pub fn boundaries(&self) -> Boundaries {
        Boundaries {
            left: self.pos.0,
            top: self.pos.1,
            right: self.pos.0 + self.dims.0 + 1.,
            bottom: self.pos.1 + self.dims.1 + 1.,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Boundaries {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}
