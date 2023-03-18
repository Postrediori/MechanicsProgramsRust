pub const G: f64 = 9.81;
pub const DT: f64 = 0.05;

pub struct PendulumModel {
    length: f64,
    theta: f64,
    theta_v: f64,
    theta_a: f64,
    g: f64,
    dtime: f64
}

impl PendulumModel {
    pub fn new() -> Self {
        Self {
            length: 1.0,
            theta: 0.0,
            theta_v: 1.0,
            theta_a: 0.0,
            g: G,
            dtime: DT
        }
    }

    pub fn set_theta(&mut self, new_theta: f64) {
        self.theta = new_theta;
        self.theta_v = 0.0;
        self.theta_a = 0.0;
    }

    pub fn theta(&self) -> f64 {
        self.theta
    }

    pub fn set_g(&mut self, new_g: f64) {
        self.g = new_g;
    }

    pub fn g(&self) -> f64 {
        self.g
    }

    pub fn set_dtime(&mut self, new_dtime: f64) {
        self.dtime = new_dtime;
    }

    pub fn dtime(&self) -> f64 {
        self.dtime
    }

    pub fn step(&mut self) {
        self.theta_a = -self.g * self.theta.sin() / self.length;
        self.theta_v += self.theta_a * self.dtime;
        self.theta += self.theta_v * self.dtime;
    }
}
