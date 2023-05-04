use super::{display::Displayable, grid::Grid, physics::Physics};

// TODO Find a way to make this less weird.

pub struct Object {
    dims: (u32, u32),
    pos: (u32, u32),

    display: Displayable,
    physics: Option<Physics>,

    last_pixels: Vec<(u32, u32)>,
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
                    (self.pos.0 + if flip.0 { self.dims.0 - x } else { x }) % grid.mat().dims().0,
                    (self.pos.1 + if flip.1 { self.dims.1 - y } else { y }) % grid.mat().dims().1,
                );
                if pixel.a == 255 {
                    grid.mat_mut()[index] = pixel;
                    self.last_pixels.push(index);
                }
            }
        }
    }

    pub fn dims(&self) -> &(u32, u32) {
        &self.dims
    }

    pub fn pos(&self) -> &(u32, u32) {
        &self.pos
    }
    pub fn pos_mut(&mut self) -> &mut (u32, u32) {
        &mut self.pos
    }

    pub fn physics(&self) -> &Physics {
        self.physics.as_ref().unwrap()
    }
    pub fn physics_mut(&mut self) -> &mut Physics {
        self.physics.as_mut().unwrap()
    }

    pub fn display(&self) -> &Displayable {
        &self.display
    }
    pub fn display_mut(&mut self) -> &mut Displayable {
        &mut self.display
    }
}

pub struct ObjectBuilder {
    pos: Option<(u32, u32)>,
    dims: Option<(u32, u32)>,

    display: Option<Displayable>,
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

    pub fn dims(mut self, dims: (u32, u32)) -> Self {
        self.dims = Some(dims);
        return self;
    }
    pub fn pos(mut self, pos: (u32, u32)) -> Self {
        self.pos = Some(pos);
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
            display: self.display.expect(
                "Object should have display, define them with ObjectBuilder::display(display).",
            ),
            physics: self.physics,
            last_pixels: Vec::new(),
        }
    }
}
