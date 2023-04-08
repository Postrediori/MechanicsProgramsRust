use std::f64::consts::PI;

pub type SurfaceFunc = fn(f64) -> f64;

pub fn linear_surface(x: f64) -> f64 {
    x - 0.5
}

pub fn sine_surface(x: f64) -> f64 {
    (2.0 * PI * x).sin()
}

pub fn cosine_surface(x: f64) -> f64 {
    (2.0 * PI * x).cos()
}

pub fn halfsine_surface(x: f64) -> f64 {
    (PI * (x + 0.5)).sin()
}
