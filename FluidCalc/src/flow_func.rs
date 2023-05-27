const K: f64 = 1.4;
const EPS: f64 = 1e-6;
const MAX_ITER: i64 = 10_000;

const SOLUTION_1: i32 = 1;
const SOLUTION_2: i32 = 2;

type Func = fn(f64)->f64;
type DFunc = fn(f64)->f64;

// Direct functions

pub type DirectFunc = fn(f64)->f64;

pub fn tau(lambda: f64) -> f64 {
    1.0 - lambda * lambda * (K - 1.0) / (K + 1.0)
}

pub fn pi(lambda: f64) -> f64 {
    tau(lambda).powf(K / (K - 1.0))
}

pub fn eps(lambda: f64) -> f64 {
    tau(lambda).powf(1.0 / (K - 1.0))
}

pub fn q(lambda: f64) -> f64 {
    eps(lambda) * lambda * (2.0 / (K + 1.0)).powf(-1.0 / (K - 1.0))
}

pub fn phi(lambda: f64) -> f64 {
    1.0 / (lambda * lambda) + 2.0 * lambda.ln()
}

pub fn y(lambda: f64) -> f64 {
    q(lambda) / pi(lambda)
}

// Inverse functions

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

pub type LambdaFuncResult = (f64, Option<f64>);
pub type InverseFunc = fn(f64)->LambdaFuncResult;

pub fn lambda_tau(tau: f64) -> LambdaFuncResult {
    (((K + 1.0) * (1.0 - tau) / (K - 1.0)).sqrt(), None)
}

pub fn lambda_pi(pi: f64) -> LambdaFuncResult {
    lambda_tau(pi.powf((K - 1.0) / K))
}

pub fn lambda_eps(eps: f64) -> LambdaFuncResult {
    lambda_tau(eps.powf(K - 1.0))
}

fn dq_dx(lambda: f64) -> f64 {
    let q1 = (2.0 / (K + 1.0)).powf(-1.0 / (K - 1.0));
    let q2 = tau(lambda).powf(1.0 / (K - 1.0) - 1.0);
    (1.0 - lambda * lambda) * q1 * q2
}

fn lambda_q_solution(qc: f64, solution: i32) -> f64 {
    let xn: f64 = match solution {
        SOLUTION_1 => 0.5,
        SOLUTION_2 => 1.5,
        _ => 0.0
    };

    iterate_lookup(qc, xn, q, dq_dx)
}

pub fn lambda_q(q: f64) -> LambdaFuncResult {
    (lambda_q_solution(q, SOLUTION_1),
        Some(lambda_q_solution(q, SOLUTION_2)))
}

fn dphi_dx(lambda: f64) -> f64 {
    2.0 * (1.0 - 1.0 / (lambda * lambda)) / lambda
}

fn lambda_phi_solution(phi_c: f64, solution: i32) -> f64 {
    let xn: f64 = match solution {
        SOLUTION_1 => 0.5,
        SOLUTION_2 => 1.5,
        _ => unreachable!(),
    };

    iterate_lookup(phi_c, xn, phi, dphi_dx)
}

pub fn lambda_phi(phi: f64) -> LambdaFuncResult {
    (lambda_phi_solution(phi, SOLUTION_1),
        Some(lambda_phi_solution(phi, SOLUTION_2)))
}

fn dy_dx(lambda: f64) -> f64 {
    let a = (K - 1.0) / (K + 1.0);
    let b = 2.0 / (K + 1.0);
    let c = b.powf(-1.0 / (K - 1.0));
    c * (1.0 + 2.0 * a * lambda * lambda) / tau(lambda)
}

pub fn lambda_y(y_c: f64) -> LambdaFuncResult {
    const Y_START: f64 = 1.0;
    (iterate_lookup(y_c, Y_START, y, dy_dx), None)
}
