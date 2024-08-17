use fltk::draw;

use crate::param_list::Parametrized;

// Base pendulum model trait
pub trait PendulumModel {
    fn label(&self) -> &'static str;
    fn time(&self) -> f64;
    fn restart(&mut self);
    fn step(&mut self);
    fn draw(&self, w: i32, h: i32, offs: &draw::Offscreen);
}

pub trait ParametrizedModel: PendulumModel + Parametrized {}
