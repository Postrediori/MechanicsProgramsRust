pub const K: f64 = 1.4;
pub const EPS: f64 = 1e-6;
const MAX_ITER: i64 = 10_000;

const SOLUTION_1: i32 = 1;
const SOLUTION_2: i32 = 2;

type Func = fn(f64) -> f64;
type DFunc = fn(f64) -> f64;

// Iterative lookup function

fn iterate_lookup(lookup_val: f64, start_x: f64, func: Func, dfunc_dx: DFunc) -> f64 {
    let mut xn = start_x;

    for _ in 0..MAX_ITER {
        let old_xn = xn;
        xn = old_xn - (func(old_xn) - lookup_val) / dfunc_dx(old_xn);
        if (xn - old_xn).abs() < EPS {
            break;
        }
    }

    xn
}

// Basic fluid fow functions

fn tau(lambda: f64) -> f64 {
    1.0 - lambda * lambda * (K - 1.0) / (K + 1.0)
}

fn eps(lambda: f64) -> f64 {
    tau(lambda).powf(1.0 / (K - 1.0))
}

// q(lambda) function and derivative

pub fn q(lambda: f64) -> f64 {
    eps(lambda) * lambda * (2.0 / (K + 1.0)).powf(-1.0 / (K - 1.0))
}

fn dq_dx(lambda: f64) -> f64 {
    let q1 = (2.0 / (K + 1.0)).powf(-1.0 / (K - 1.0));
    let q2 = tau(lambda).powf(1.0 / (K - 1.0) - 1.0);
    (1.0 - lambda * lambda) * q1 * q2
}

// Get single solution for q(lambda)

fn lambda_q_solution(qc: f64, solution: i32) -> f64 {
    let xn: f64 = match solution {
        SOLUTION_1 => 0.5,
        SOLUTION_2 => 1.5,
        _ => 0.0,
    };

    iterate_lookup(qc, xn, q, dq_dx)
}

// Get two solutions for q(lambda)

pub fn lambda_q(q: f64) -> (f64, f64) {
    (
        lambda_q_solution(q, SOLUTION_1),
        lambda_q_solution(q, SOLUTION_2),
    )
}
