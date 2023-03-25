use std::f64::consts::PI;

pub type SurfaceFunc = fn(f64, f64, f64) -> f64;

pub fn linear_surface(x: f64, delta: f64, epsilon: f64) -> f64 {
    epsilon * (x / delta - 0.5)
}

pub fn sine_surface(x: f64, delta: f64, epsilon: f64) -> f64 {
    epsilon * ((2.0 * PI * x / delta).sin())
}

pub fn cosine_surface(x: f64, delta: f64, epsilon: f64) -> f64 {
    epsilon * ((2.0 * PI * x / delta).cos())
}
