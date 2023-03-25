use std::f64::consts::PI;
use crate::surface_functions;

const DEFAULT_G: f64 = 9.81;
const DEFAULT_DELTA: f64 = 1.0;
const DEFAULT_EPSILON: f64 = 0.1;
const DEFAULT_H: f64 = 0.3;
const DEFAULT_DTIME: f64 = 0.01;

const DEFAULT_XN: usize = 30;
const DEFAULT_ZN: usize = 15;
const DEFAULT_MAXN: usize = 50;

// Describes a moveable point
// x,z: coordinates
// x0,z0: original coordinates

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub z: f64,
    x0: f64,
    z0: f64
}

type Points = Vec<Point>;


// Model of water cross-section
pub struct WaveModel {
    // Gravitational constant
    pub g: f64,
    // Model parameters
    pub delta: f64,
    pub epsilon: f64,
    pub h: f64,
    // Current time, time delta for single step and
    // number of frame since the start of simulation
    pub time: f64,
    pub dtime: f64,
    frame: i32,
    // Number of points along each coordinate
    pub xn: usize,
    pub zn: usize,
    pub maxn: usize,
    // Parameters vectors
    kn: Vec<f64>,
    an: Vec<f64>,
    sigman2: Vec<f64>,
    sigman: Vec<f64>,
    // Points data vector
    pub points: Points,
    // Current surface function
    surface_func: surface_functions::SurfaceFunc,
}

impl WaveModel {
    pub fn make_model() -> Self {
        Self {
            g: DEFAULT_G,
            delta: DEFAULT_DELTA,
            epsilon: DEFAULT_EPSILON,
            h: DEFAULT_H,
            time: 0.0,
            dtime: DEFAULT_DTIME,
            frame: 0,
            xn: DEFAULT_XN,
            zn: DEFAULT_ZN,
            maxn: DEFAULT_MAXN,
            kn: vec![0.0; DEFAULT_MAXN],
            an: vec![0.0; DEFAULT_MAXN],
            sigman2: vec![0.0; DEFAULT_MAXN],
            sigman: vec![0.0; DEFAULT_MAXN],
            points: vec![Point { x: 0.0, z: 0.0, x0: 0.0, z0: 0.0 }; DEFAULT_XN * DEFAULT_ZN],
            surface_func: surface_functions::linear_surface,
        }
    }

    pub fn reset(&mut self) {
        self.time = 0.0;
        self.frame = 0;

        self.calc_coeffs();

        let dx = self.delta / ((self.xn - 1) as f64);
        let dz = self.h / ((self.zn - 1) as f64);

        for idx in 0..(self.xn * self.zn) {
            let i = idx / self.zn;
            let j = idx % self.zn;

            let x0 = dx * (i as f64);
            let z0 = -dz * (j as f64);
            let x = x0 + self.f_x(x0, z0, self.time);
            let z = z0 - self.f_z(x0, z0, self.time);

            self.points[idx] = Point { x: x, z: z, x0: x0, z0: z0 };
        }
    }

    pub fn step(&mut self) {
        self.frame += 1;
        self.time = (self.frame as f64) * self.dtime;
        for idx in 0..(self.xn * self.zn) {
            let p = self.points[idx];

            let x = p.x0 + self.f_x(p.x0, p.z0, self.time);
            let z = p.z0 - self.f_z(p.x0, p.z0, self.time);

            self.points[idx] = Point { x: x, z: z, x0: p.x0, z0: p.z0 };
        }
    }

    pub fn set_surface_func(&mut self, n: i32) {
        match n {
        0 => { self.surface_func = surface_functions::linear_surface; }
        1 => { self.surface_func = surface_functions::sine_surface; }
        2 => { self.surface_func = surface_functions::cosine_surface; }
        _ => { println!("Unknown surface type!"); }
        }
    }

    // Math functions
    fn fourier_n(&self, d: f64, n: usize) -> f64 {
        const MAX_ITER: usize = 1_000;

        let mut fi = vec![0.0; MAX_ITER+1];

        let minx = 0.0;
        let maxx = d;
        let dx = (maxx - minx) / (MAX_ITER as f64);
        for i in 0..MAX_ITER+1 {
            let xi = minx + dx * (i as f64);
            fi[i] = (self.surface_func)(xi, self.delta, self.epsilon) * (self.kn[n as usize] * xi).cos();
        }

        let k = MAX_ITER / 2;
        let s1 = fi[0] + fi[MAX_ITER];
        let mut s2 = 0.0;
        let mut s4 = 0.0;
        for i in 2..k+1 {
            s2 += fi[(i - 1) * 2 - 1 as usize];
        }
        for i in 1..k+1 {
            s4 += fi[(i - 1) * 2 as usize];
        }

        let s = (s1 + 2.0 * s2 + 4.0 * s4) * dx / 3.0;

        2.0 * s / d
    }

    fn calc_a(&mut self) {
        for i in 0..self.maxn {
            self.an[i] = self.fourier_n(self.delta, i);
        }
    }

    fn calc_coeffs(&mut self) {
        for i in 0..self.maxn {
            self.kn[i] = PI * ((i + 1) as f64) / self.delta;
            self.sigman2[i] = self.g * self.kn[i] * (self.kn[i] * self.h).tanh();
            self.sigman[i] = self.sigman2[i].sqrt();
        }

        self.calc_a();
    }

    fn g_xn(&self, z0: f64, t: f64, n: usize) -> f64 {
        -self.kn[n] * self.an[n] * (self.kn[n] * (z0 + self.h)).cosh() *
            (self.sigman[n] * t).cos() / (self.sigman2[n] * (self.kn[n] * self.h).cosh())
    }

    fn g_zn(&self, z0: f64, t: f64, n: usize) -> f64 {
        -self.kn[n] * self.an[n] * (self.kn[n] * (z0 + self.h)).sinh() *
            (self.sigman[n] * t).cos() / (self.sigman2[n] * (self.kn[n] * self.h).cosh())
    }

    fn f_x(&self, x0: f64, z0: f64, t: f64) -> f64 {
        let mut sum = 0.0;
        for i in 0..self.maxn {
            sum += self.g_xn(z0, t, i) * ((self.kn[i] * x0).sin());
        }
        self.g * sum
    }

    fn f_z(&self, x0: f64, z0: f64, t: f64) -> f64 {
        let mut sum = 0.0;
        for i in 0..self.maxn {
            sum += self.g_zn(z0, t, i) * ((self.kn[i] * x0).cos());
        }
        self.g * sum
    }
}
