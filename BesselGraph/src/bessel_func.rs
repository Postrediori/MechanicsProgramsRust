const GAMMA: f64 = 0.5772156649;
const PI: f64 = 3.1415926;
const DELTA: f64 = 1e-6;
const MAX_ITER: i64 = 10_000;

type Constants = [f64];
type IntegrationFunc = fn(f64, &Constants) -> f64;

fn integrate(x1: f64, x2: f64, scale: i64, f: IntegrationFunc, c: &Constants) -> f64 {
    let div_count = scale * 2 + 1; // Always odd number

    let dx: f64 = (x2 - x1) / (div_count as f64);
    let mut s: f64 = 0.0;
    for i in 0..div_count {
        let x = x1 + dx * (i as f64);
        let y = f(x, c);
        let m = if i > 0 && i < (div_count - 1) { ((i % 2 + 1) * 2) as f64} else { 1.0 };
        s += y * m;
    }

    s * dx / 3.0
}

// Bessel Function of the Second Kind, 0-th order
// Method 1: Integration

fn d_y0_1(theta: f64, c: &Constants) -> f64 {
    let x: f64 = c[0];

    let sin_theta: f64 = theta.sin();
    (x * theta.cos()).cos() * (GAMMA + (2.0 * x * sin_theta * sin_theta).ln())
}

pub fn y0_1(x: f64) -> f64 {
    integrate(DELTA, PI / 2.0, MAX_ITER, d_y0_1, &[x]) * 4.0 / (PI * PI)
}

// Bessel Function of the Second Kind, 0-th order
// Method 2: Series

// Bessel Function of the First Kind, 0-th order
fn d_j0(theta: f64, c: &Constants) -> f64 {
    let x: f64 = c[0];
    (theta.sin() * x).cos()
}

fn j0(x: f64) -> f64 {
    integrate(0.0, PI, MAX_ITER, d_j0, &[x]) / PI
}

// Bessel Function of the Second Kind, 0-th order infinite series
pub fn y0_2(x: f64) -> f64 {
    let mut s: f64 = 0.0;

    let dm: f64 = x * x / 4.0;
    let mut m: f64 = dm;
    let mut n: f64 = 1.0;
    let mut p: f64 = 1.0;

    for dn in 2..MAX_ITER {
        let a = p * m / (n * n);
        s += a;
        
        if a.abs() < DELTA {
            break;
        }

        m *= -dm;
        n *= dn as f64;
        p += 1.0 / (dn as f64);
    }

    (((x / 2.0).ln() + GAMMA) * j0(x) + s) * 2.0 / PI
}
