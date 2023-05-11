use crate::vec::Vec2;

pub struct Physics {
    s: Vec2<f32>,  // position in px
    v: Vec2<f32>,  // velocity in px/s
    a: Vec2<f32>,  // acceleration in px/s^2
    m: f32,        // mass in mu (mass unit)
    w: f32,        // weight in fu (force unit)
    tf: Vec2<f32>, // total force in fu
}

impl Physics {
    pub fn new<S, V>(s0: S, v0: V, m: f32, g: f32) -> Self
    where
        S: Into<Vec2<f32>>,
        V: Into<Vec2<f32>>,
    {
        Self {
            s: s0.into(),
            v: v0.into(),
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

    /// Apply a new force on the object, updates the total force.
    pub fn apply_force<F>(&mut self, force: F)
    where
        F: Into<Vec2<f32>>,
    {
        self.tf = self.tf + force.into();
    }
    /// Set the total force to the weight of the object.
    pub fn set_tf_to_w(&mut self) {
        self.tf = Vec2(0., self.w);
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
    pub fn set_v<V>(&mut self, v: V)
    where
        V: Into<Vec2<f32>>,
    {
        self.v = v.into();
    }

    /// The total force applied on the object, the weight is
    /// included.
    pub fn tf(&self) -> &Vec2<f32> {
        &self.tf
    }
    pub fn tf_mut(&mut self) -> &mut Vec2<f32> {
        &mut self.tf
    }
    // Position of the object in px
    pub fn s(&self) -> &Vec2<f32> {
        &self.s
    }
    pub fn s_mut(&mut self) -> &mut Vec2<f32> {
        &mut self.s
    }
    // Velocity of the object in px/s
    pub fn v(&self) -> &Vec2<f32> {
        &self.v
    }
    pub fn v_mut(&mut self) -> &mut Vec2<f32> {
        &mut self.v
    }
    // Acceleration of the object in px/s^2
    pub fn a(&self) -> &Vec2<f32> {
        &self.a
    }
    pub fn a_mut(&mut self) -> &mut Vec2<f32> {
        &mut self.a
    }
    // Weight of the object in fu (force unit)
    pub fn w(&self) -> &f32 {
        &self.w
    }
}
