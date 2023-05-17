use crate::vec::Vec2;

pub struct Object {
    pos: Vec2<i32>,
    dims: Vec2<i32>,
}

impl Object {
    pub fn new<P, D>(pos: P, dims: D) -> Self
    where
        P: Into<Vec2<i32>>,
        D: Into<Vec2<i32>>,
    {
        Object {
            pos: pos.into(),
            dims: dims.into(),
        }
    }

    pub fn colliding_with(&self, other: &Object) -> bool {
        let (min1, max1, min2, max2) = (
            self.pos,
            (
                self.pos.0 + self.dims.0 as i32 + 1,
                self.pos.1 + self.dims.1 as i32 + 1,
            ),
            other.pos,
            (
                other.pos.0 + other.dims.0 as i32 + 1,
                other.pos.1 + other.dims.1 as i32 + 1,
            ),
        );

        !(max1.0 < min2.0 || min1.0 > max2.0 || max1.1 < min2.1 || min1.1 > max2.1)
    }

    pub fn pos(&self) -> &Vec2<i32> {
        &self.pos
    }
    pub fn pos_mut(&mut self) -> &mut Vec2<i32> {
        &mut self.pos
    }

    pub fn dims(&self) -> &Vec2<i32> {
        &self.dims
    }
    pub fn dims_mut(&mut self) -> &mut Vec2<i32> {
        &mut self.dims
    }
}
