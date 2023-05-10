use super::{display::Drawable, grid::Grid, physics::Physics};

// TODO Find a way to make this less weird.

pub struct Object {
    dims: (usize, usize),
    pos: (usize, usize),

    display: Drawable,
    physics: Option<Physics>,

    last_pixels: Vec<(usize, usize)>,
}

impl Object {
    pub fn draw(&mut self, grid: &mut Grid) {
        self.last_pixels.clear();

        let display = &mut self.display;

        for x in 0..display.dims().0 {
            for y in 0..display.dims().1 {
                let pixel = display.current()[(x, y)];
                let flip = display.flip();
                let index = (
                    (self.pos.0 + if flip.0 { self.dims.0 - x } else { x }) % grid.dims().0,
                    (self.pos.1 + if flip.1 { self.dims.1 - y } else { y }) % grid.dims().1,
                );
                if pixel.a == 255 {
                    grid.mat_mut()[index] = pixel;
                    self.last_pixels.push(index);
                }
            }
        }
    }

    pub fn dims(&self) -> &(usize, usize) {
        &self.dims
    }

    pub fn pos(&self) -> &(usize, usize) {
        &self.pos
    }
    pub fn pos_mut(&mut self) -> &mut (usize, usize) {
        &mut self.pos
    }

    pub fn physics(&self) -> &Physics {
        self.physics.as_ref().unwrap()
    }
    pub fn physics_mut(&mut self) -> &mut Physics {
        self.physics.as_mut().unwrap()
    }

    pub fn display(&self) -> &Drawable {
        &self.display
    }
    pub fn display_mut(&mut self) -> &mut Drawable {
        &mut self.display
    }
}

pub struct ObjectBuilder {
    pos: Option<(usize, usize)>,
    dims: Option<(usize, usize)>,

    display: Option<Drawable>,
    physics: Option<Physics>,
}

impl ObjectBuilder {
    pub fn new() -> Self {
        ObjectBuilder {
            pos: None,
            dims: None,
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
    pub fn display<D>(mut self, display: D) -> Self
    where
        D: Into<Drawable>,
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
            display: self.display.expect(
                "Object should have display, define them with ObjectBuilder::display(display).",
            ),
            physics: self.physics,
            last_pixels: Vec::new(),
        }
    }
}
