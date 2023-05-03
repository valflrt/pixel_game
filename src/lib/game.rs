use crate::{color::Color, mat::Mat, vec::Vec2};

pub use self::display::*;

mod display;

#[derive(PartialEq)]
pub enum Move {
    Forward,
    Backward,
    None,
}

pub struct Grid {
    default_color: Color,
    mat: Mat<Color>,
}

impl Grid {
    pub fn new(default_color: Color, dims: (usize, usize)) -> Self {
        Grid {
            default_color,
            mat: Mat::new(default_color, dims),
        }
    }

    pub fn draw(&self, pixels: &mut [u8]) {
        for (c, pix) in self.mat.iter().zip(pixels.chunks_exact_mut(4)) {
            pix.copy_from_slice(&c.to_bytes());
        }
    }
}

pub struct Object {
    pub pos: (usize, usize),
    pub dims: (usize, usize),
    pub direction: (Move, Move),

    pub display: Displayable,
    pub physics: Option<Physics>,

    last_pixels: Vec<(usize, usize)>,
}

impl Object {
    pub fn draw(&mut self, grid: &mut Grid) {
        for index in &self.last_pixels {
            grid.mat[*index] = grid.default_color;
        }
        self.last_pixels.clear();

        let display = &mut self.display;

        for x in 0..display.dims().0 {
            for y in 0..display.dims().1 {
                let pixel = display.current()[(x, y)];
                let flip = display.flip();
                let index = (
                    (self.pos.0 + if flip.0 { self.dims.0 - x } else { x }) % grid.mat.dims().0,
                    (self.pos.1 + if flip.1 { self.dims.1 - y } else { y }) % grid.mat.dims().1,
                );
                if pixel.a == 255 {
                    grid.mat[index] = pixel;
                    self.last_pixels.push(index);
                }
            }
        }
    }
}

pub struct ObjectBuilder {
    pos: Option<(usize, usize)>,
    dims: Option<(usize, usize)>,
    direction: Option<(Move, Move)>,

    display: Option<Displayable>,
    physics: Option<Physics>,
}

impl ObjectBuilder {
    pub fn new() -> Self {
        ObjectBuilder {
            pos: None,
            dims: None,
            direction: None,
            display: None,
            physics: None,
        }
    }

    pub fn dims(mut self, dims: (usize, usize)) -> Self {
        self.dims = Some(dims);
        return self;
    }
    pub fn pos(mut self, pos: (usize, usize)) -> Self {
        self.pos = Some(pos);
        return self;
    }
    pub fn direction(mut self, direction: (Move, Move)) -> Self {
        self.direction = Some(direction);
        return self;
    }
    pub fn display<D>(mut self, display: D) -> Self
    where
        D: Into<Displayable>,
    {
        self.display = Some(display.into());
        return self;
    }
    pub fn physics(mut self, physics: Physics) -> Self {
        self.physics = Some(physics);
        return self;
    }

    pub fn build(self) -> Object {
        Object {
            pos: self.pos.unwrap_or((0, 0)),
            dims: self
                .dims
                .expect("Object should have dims, define it with ObjectBuilder::dims(dims)."),
            direction: self.direction.unwrap_or((Move::None, Move::None)),
            display: self.display.expect(
                "Object should have display, define them with ObjectBuilder::display(display).",
            ),
            physics: self.physics,
            last_pixels: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hitbox {
    pos: (usize, usize),
    dims: (usize, usize),
}

pub struct Physics {
    s: Vec2<f32>,  // position in px
    v: Vec2<f32>,  // velocity in px/s
    a: Vec2<f32>,  // acceleration in px/s^2
    m: f32,        // mass in mu (mass unit)
    w: f32,        // weight in fu (force unit)
    tf: Vec2<f32>, // total force in fu
}

impl Physics {
    pub fn new(s0: Vec2<f32>, v0: Vec2<f32>, m: f32, g: f32) -> Self {
        Self {
            s: s0,
            v: v0,
            a: Vec2(0., 0.),
            tf: Vec2(0., 0.),
            m,
            w: m * g,
        }
    }

    /// Update positon, velocity and acceleration.
    pub fn update(&mut self, dt: f32) {
        let a = self.tf / self.m; // Newton's second law
        let v = self.a * dt;
        let s = self.v * dt + (self.a * dt.powi(2)) / 2.;

        self.a += a;
        self.v += v;
        self.s += s;
    }

    /// Apply a new force on the object, updates the total
    /// force.
    pub fn apply_force(&mut self, force: Vec2<f32>) {
        self.tf = self.tf + force;
    }
    /// Set the total force to the weight of the object.
    pub fn set_tf_to_w(&mut self) {
        self.tf = Vec2(0., self.w);
    }
    /// Reset the total force to 0.
    pub fn reset_tf(&mut self) {
        self.tf = Vec2(0., 0.);
    }

    /// The total force applied on the object, the weight is
    /// included.
    pub fn tf(&self) -> &Vec2<f32> {
        &self.tf
    }
    // Position of the object in px
    pub fn s(&self) -> &Vec2<f32> {
        &self.s
    }
    // Velocity of the object in px/s
    pub fn v(&self) -> &Vec2<f32> {
        &self.v
    }
    // Acceleration of the object in px/s^2
    pub fn a(&self) -> &Vec2<f32> {
        &self.a
    }
    // Weight of the object in fu (force unit)
    pub fn w(&self) -> &f32 {
        &self.w
    }
}
