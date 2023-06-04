use crate::vec2::Vec2;

pub struct Physics {
    m: f64, // mass in mu (mass unit)
    g: f64, // gravity strength in fu/mu

    pos: Vec2, // position in px
    v: Vec2,   // velocity in px/s
    a: Vec2,   // acceleration in px/s^2

    tf: Vec2, // total force in fu (force unit)
}

impl Physics {
    pub fn new(pos: Vec2, v: Vec2, m: f64, g: f64) -> Self {
        Self {
            pos,
            v,
            a: Vec2::ZERO,
            tf: Vec2::ZERO,
            m,
            g,
        }
    }

    /// Update positon, velocity and acceleration.
    pub fn update(&mut self, dt: f64) {
        self.a = self.tf / self.m; // Newton's second law

        self.v += self.a * dt;
        self.pos += self.v * dt + (self.a * dt.powi(2)) / 2.;
    }

    /// Apply a new force on the object, updates the total force.
    pub fn apply_force(&mut self, force: Vec2) {
        self.tf += force;
    }
    /// Set the total force to the weight of the object.
    pub fn set_tf_to_w(&mut self) {
        self.tf = Vec2(0., self.m * self.g);
    }
    /// Reset the total force to 0.
    pub fn reset_tf(&mut self) {
        self.tf = Vec2(0., 0.);
    }
    /// Reset acceleration, velocity and total force to 0.
    pub fn reset_all(&mut self) {
        self.a = Vec2(0., 0.);
        self.v = Vec2(0., 0.);
        self.tf = Vec2(0., 0.);
    }

    /// The total force applied on the object, the weight is
    /// included.
    pub fn tf(&self) -> &Vec2 {
        &self.tf
    }
    pub fn tf_mut(&mut self) -> &mut Vec2 {
        &mut self.tf
    }
    /// Position of the object in px
    pub fn pos(&self) -> &Vec2 {
        &self.pos
    }
    pub fn pos_mut(&mut self) -> &mut Vec2 {
        &mut self.pos
    }
    /// Velocity of the object in px/s
    pub fn v(&self) -> &Vec2 {
        &self.v
    }
    pub fn v_mut(&mut self) -> &mut Vec2 {
        &mut self.v
    }
    /// Acceleration of the object in px/s^2
    pub fn a(&self) -> &Vec2 {
        &self.a
    }
    pub fn a_mut(&mut self) -> &mut Vec2 {
        &mut self.a
    }

    pub fn m(&self) -> &f64 {
        &self.m
    }
    pub fn m_mut(&mut self) -> &mut f64 {
        &mut self.m
    }

    pub fn g(&self) -> &f64 {
        &self.g
    }
    pub fn g_mut(&mut self) -> &mut f64 {
        &mut self.g
    }

    pub fn w(&self) -> f64 {
        self.m * self.g
    }
}
