use super::grid::Grid;

// TODO Find a way to make this less weird.

pub struct Object<F>
where
    F: Fn(&Grid),
{
    pos: (usize, usize),
    update: F,
}

impl<F> Object<F>
where
    F: Fn(&Grid),
{
    pub fn draw(&self, grid: &mut Grid) {
        (self.update)(grid);
    }

    pub fn pos(&self) -> &(usize, usize) {
        &self.pos
    }
    pub fn pos_mut(&mut self) -> &mut (usize, usize) {
        &mut self.pos
    }
}

pub struct ObjectBuilder<F>
where
    F: Fn(&Grid),
{
    pos: Option<(usize, usize)>,
    update: Option<F>,
}

impl<F> ObjectBuilder<F>
where
    F: Fn(&Grid),
{
    pub fn new() -> Self {
        ObjectBuilder {
            pos: None,
            update: None,
        }
    }

    /// Define the position of the Object.
    pub fn pos(mut self, pos: (usize, usize)) -> Self {
        self.pos = Some(pos);
        return self;
    }
    /// Define the object's update function.
    pub fn update_fn(mut self, update_fn: F) -> Self {
        self.update = Some(update_fn);
        return self;
    }

    /// Build the Object.
    pub fn build(self) -> Object<F> {
        Object {
            pos: self.pos.unwrap_or((0, 0)),
            update: self.update.expect(
                "Object should have an update function, define it with ObjectBuilder::update_fn(...).",
            ),
        }
    }
}
