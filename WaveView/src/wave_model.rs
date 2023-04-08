use std::f64::consts::PI;
use crate::surface_functions;

#[cfg(debug_assertions)]
use std::time::SystemTime;

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

#[derive(Copy, Clone)]
struct Particle {
    k: f64,
    a: f64,
    sigma2: f64,
    sigma: f64,
}

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
    particles: Vec<Particle>,
    // Points data vector
    pub points: Points,
    // Current surface function
    surface_func: surface_functions::SurfaceFunc,
    #[cfg(debug_assertions)]
    benchmarks: Vec<u128>,
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
            particles: vec![Particle{k: 0.0, a: 0.0, sigma2: 0.0, sigma: 0.0}; DEFAULT_MAXN],
            points: vec![Point { x: 0.0, z: 0.0, x0: 0.0, z0: 0.0 }; DEFAULT_XN * DEFAULT_ZN],
            surface_func: surface_functions::linear_surface,
            #[cfg(debug_assertions)]
            benchmarks: vec![],
        }
    }

    pub fn reset(&mut self) {
        self.time = 0.0;
        self.frame = 0;
        
        #[cfg(debug_assertions)]
        self.benchmarks.clear();

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
        #[cfg(debug_assertions)]
        let t_init = SystemTime::now();

        self.frame += 1;
        self.time = (self.frame as f64) * self.dtime;
        for idx in 0..(self.xn * self.zn) {
            let p = self.points[idx];

            self.points[idx].x = p.x0 + self.f_x(p.x0, p.z0, self.time);
            self.points[idx].z = p.z0 - self.f_z(p.x0, p.z0, self.time);
        }

        #[cfg(debug_assertions)]
        self.benchmarks.push(t_init.elapsed().unwrap().as_micros());
    }

    pub fn set_surface_func(&mut self, n: i32) {
        self.surface_func = match n {
        0 => { surface_functions::linear_surface }
        1 => { surface_functions::sine_surface }
        2 => { surface_functions::cosine_surface }
        3 => { surface_functions::halfsine_surface }
        _ => { eprintln!("Unknown surface type!"); |_| { 0.0 } }
        };
    }

    // Math functions
    fn fourier_n(&self, d: f64, p_k: f64) -> f64 {
        const MAX_ITER: usize = 1_000;

        let minx = 0.0;
        let maxx = d;
        let dx = (maxx - minx) / (MAX_ITER as f64);

        let s: f64 = (0..MAX_ITER+1).map(|i| {
            // Calculate plot points
            let xi = minx + dx * (i as f64);
            let yi = self.epsilon * (self.surface_func)(xi / self.delta) * (p_k * xi).cos();

            // Calculate coefficients for Simpson's rule integration
            let k = match i {
                0 | MAX_ITER => { 1 },
                _ => { 2 + (i % 2) * 2 }
            };

            yi * (k as f64)
        }).sum::<f64>() * dx / 3.0;

        2.0 * s / d
    }

    fn calc_coeffs(&mut self) {
        self.particles = (0..self.maxn).map(|i| {
            let k =  PI * ((i + 1) as f64) / self.delta;
            let a = self.fourier_n(self.delta, k);
            let sigma2 = self.g * k * (k * self.h).tanh();
            let sigma = sigma2.sqrt();
            Particle { k, a, sigma2, sigma }
        }).collect();
    }

    fn g_xn(z0: f64, p: &Particle, h: f64, t: f64) -> f64 {
        -p.k * p.a * (p.k * (z0 + h)).cosh() *
            (p.sigma * t).cos() / (p.sigma2 * (p.k * h).cosh())
    }

    fn g_zn(z0: f64, p: &Particle, h: f64, t: f64) -> f64 {
        -p.k * p.a * (p.k * (z0 + h)).sinh() *
            (p.sigma * t).cos() / (p.sigma2 * (p.k * h).cosh())
    }

    fn f_x(&self, x0: f64, z0: f64, t: f64) -> f64 {
        self.g * self.particles.iter().map(|p| {
            let g_xn = WaveModel::g_xn(z0, &p, self.h, t);
            g_xn * (p.k * x0).sin()
        }).sum::<f64>()
    }

    fn f_z(&self, x0: f64, z0: f64, t: f64) -> f64 {
        self.g * self.particles.iter().map(|p| {
            let g_zn = WaveModel::g_zn(z0, &p, self.h, t);
            g_zn * (p.k * x0).cos()
        }).sum::<f64>()
    }
    
    #[cfg(debug_assertions)]
    pub fn benchmark(&self) -> u128 {
        (self.benchmarks.iter().sum::<u128>() as f64 / self.benchmarks.len() as f64) as u128
    }
}
