use std::path::Path;

use crate::{color::Color, mat::Mat, vec::Vec2};

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
}

pub struct StaticObject {
    pub object: Object,
    image: Mat<Color>,
    last_pixels: Vec<(usize, usize)>,
}

impl StaticObject {
    pub fn from_file(path: &str, dims: (usize, usize)) -> Self {
        let image = image::open(path).unwrap().to_rgba8();
        let image_pixels: Vec<Color> = image
            .as_raw()
            .chunks(4)
            .map(|v| Color {
                r: v[0],
                g: v[1],
                b: v[2],
                a: v[3],
            })
            .collect();
        StaticObject {
            object: Object {
                pos: (0, 0),
                dims,
                direction: (Move::None, Move::None),
            },
            image: Mat::from_vec(image_pixels, dims),
            last_pixels: Vec::new(),
        }
    }

    pub fn from_color(color: Color, dims: (usize, usize)) -> Self {
        StaticObject {
            object: Object {
                pos: (0, 0),
                dims,
                direction: (Move::None, Move::None),
            },
            image: Mat::new(color, dims),
            last_pixels: Vec::new(),
        }
    }

    pub fn draw(&mut self, grid: &mut Grid) {
        for index in &self.last_pixels {
            grid.mat[*index] = grid.default_color;
        }
        self.last_pixels.clear();
        let object = &self.object;
        for x in 0..object.dims.0 {
            for y in 0..object.dims.1 {
                let pixel = &self.image[(x, y)];
                let index = (
                    (object.pos.0 + x) % grid.mat.dims().0,
                    (object.pos.1 + y) % grid.mat.dims().1,
                );
                if pixel.a == 255 {
                    grid.mat[index] = *pixel;
                    self.last_pixels.push(index);
                }
            }
        }
    }
}

pub struct AnimatedObject {
    pub pos: (usize, usize),
    pub dims: (usize, usize),
    pub direction: (Move, Move),
    pub animation: Animation,
    last_pixels: Vec<(usize, usize)>,
    // forces: Vec<(usize, usize)>,
    // hitbox: Hitbox,
}

impl AnimatedObject {
    pub fn new(animation: Animation, dims: (usize, usize) /*, hitbox: Hitbox*/) -> Self {
        assert_eq!(dims, animation.dims);
        AnimatedObject {
            pos: (0, 0),
            dims,
            animation,
            // hitbox,
            // forces: Vec::new(),
            direction: (Move::None, Move::None),
            last_pixels: Vec::new(),
        }
    }

    pub fn next_frame(&mut self) {
        self.animation.next();
    }

    pub fn draw(&mut self, grid: &mut Grid) {
        for index in &self.last_pixels {
            grid.mat[*index] = grid.default_color;
        }
        self.last_pixels.clear();
        for x in 0..self.dims.0 {
            for y in 0..self.dims.1 {
                let pixel = &self.animation.current_frame()[(x, y)];
                let index = (
                    (self.pos.0
                        + if self.animation.flip.0 {
                            self.dims.0 - x
                        } else {
                            x
                        })
                        % grid.mat.dims().0,
                    (self.pos.1
                        + if self.animation.flip.1 {
                            self.dims.1 - y
                        } else {
                            y
                        })
                        % grid.mat.dims().1,
                );
                if pixel.a == 255 {
                    grid.mat[index] = *pixel;
                    self.last_pixels.push(index);
                }
            }
        }
    }
}

pub struct Animation {
    state: usize,
    frame: usize,
    // texture: Mat<Color>,
    states: Vec<Vec<Mat<Color>>>,
    pub flip: (bool, bool),
    dims: (usize, usize),
}

impl Animation {
    pub fn from_files<T>(paths: &[&[T]], dims: (usize, usize)) -> Self
    where
        T: AsRef<Path>,
    {
        let mut animations = Vec::new();
        for frames_paths in paths {
            let mut frames = Vec::new();
            for path in *frames_paths {
                let image = image::open(path).unwrap().to_rgba8();
                let image_pixels: Vec<Color> = image
                    .as_raw()
                    .chunks(4)
                    .map(|v| Color {
                        r: v[0],
                        g: v[1],
                        b: v[2],
                        a: v[3],
                    })
                    .collect();
                frames.push(Mat::from_vec(image_pixels, dims));
            }
            animations.push(frames)
        }
        Animation {
            state: 0,
            frame: 0,
            states: animations,
            flip: (false, false),
            dims,
        }
    }

    pub fn next(&mut self) {
        self.frame = (self.frame + 1) % self.states[self.state].len();
    }

    pub fn current_frame(&self) -> &Mat<Color> {
        &self.states[self.state][self.frame]
    }

    pub fn set_state(&mut self, state: usize) {
        if self.state != state {
            self.frame = 0;
            self.state = state;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hitbox {
    pos: (usize, usize),
    dims: (usize, usize),
}

pub struct Physics {
    pos: Vec2<f32>,
    v: Vec2<f32>,
    a: Vec2<f32>,

    f: Vec2<f32>,

    g: f32,
    m: f32,
}

impl Physics {
    pub fn new(pos: Vec2<f32>, v: Vec2<f32>, m: f32, g: f32) -> Self {
        Self {
            pos,
            v,
            a: Vec2(0., 0.),
            f: Vec2(0., m * g),
            g,
            m,
        }
    }

    /// Update positon, velocity and acceleration.
    pub fn update(&mut self, dt: f32) {
        self.a = self.f / self.m;
        self.pos = self.pos + self.v * dt + (self.a * dt.powi(2)) / 2.;
        self.v = self.v + self.a * dt;
    }

    /// Apply a new force on the object.
    pub fn add_force(&mut self, force: Vec2<f32>) {
        self.f = self.f + force;
    }

    pub fn remove_force(&mut self, force: Vec2<f32>) {
        self.f = self.f - force;
    }

    /// The sum of the forces applied on the object, the weight
    /// is included.
    pub fn f(&self) -> &Vec2<f32> {
        &self.f
    }

    pub fn pos(&self) -> &Vec2<f32> {
        &self.pos
    }

    pub fn v(&self) -> &Vec2<f32> {
        &self.v
    }

    pub fn a(&self) -> &Vec2<f32> {
        &self.a
    }
}
