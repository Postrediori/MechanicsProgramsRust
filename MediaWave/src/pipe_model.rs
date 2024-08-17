const DEFAULT_A: f64 = 1.0;
const DEFAULT_RHO: f64 = 1.0;
const DEFAULT_SIGMA: f64 = 0.1;

pub struct BoundaryCondition {
    b: f64,
    c: f64,
}

pub const BOUNDARY_SEALED: BoundaryCondition = BoundaryCondition { b: 0.0, c: 1.0 };
pub const BOUNDARY_OPEN: BoundaryCondition = BoundaryCondition { b: 1.0, c: 0.0 };

const DEFAULT_CONDITION_L: BoundaryCondition = BOUNDARY_SEALED;
const DEFAULT_CONDITION_R: BoundaryCondition = BOUNDARY_SEALED;

type InitialFunc = fn(f64) -> f64;

fn f_initial_u(id: &str) -> InitialFunc {
    const UN_0: f64 = 0.0;
    const UN_1: f64 = 1.0;

    match id {
        "▄▄▄▄▄▄" => |_| UN_0,
        "██▄▄██" => |x| {
            if x * 3.0 < 1.0 || x * 3.0 > 2.0 {
                UN_1
            } else {
                UN_0
            }
        },
        "██▄▄▄▄" => |x| if x * 3.0 < 1.0 { UN_1 } else { UN_0 },
        "▄▄██▄▄" => |x| {
            if x * 3.0 > 1.0 && x * 3.0 < 2.0 {
                UN_1
            } else {
                UN_0
            }
        },
        "▄▄▄▄██" => |x| if x * 3.0 > 2.0 { UN_1 } else { UN_0 },
        "▄▄▄███" => |x| if x < 0.5 { -UN_1 } else { UN_1 },
        "███▄▄▄" => |x| if x < 0.5 { UN_1 } else { -UN_1 },
        _ => {
            eprintln!("Unknown initial conditions id {}", id);
            |_| UN_0
        }
    }
}

fn f_initial_p(id: &str) -> InitialFunc {
    const PN_0: f64 = 0.0;
    const PN_1: f64 = 1.0;

    match id {
        "▄▄▄▄▄▄" => |_| PN_0,
        "██▄▄██" => |x| {
            if x * 3.0 < 1.0 || x * 3.0 > 2.0 {
                PN_1
            } else {
                PN_0
            }
        },
        "██▄▄▄▄" => |x| if x * 3.0 < 1.0 { PN_1 } else { PN_0 },
        "▄▄██▄▄" => |x| {
            if x * 3.0 > 1.0 && x * 3.0 < 2.0 {
                PN_1
            } else {
                PN_0
            }
        },
        "▄▄▄▄██" => |x| if x * 3.0 > 2.0 { PN_1 } else { PN_0 },
        "▄▄▄███" => |x| if x < 0.5 { -PN_1 } else { PN_1 },
        "███▄▄▄" => |x| if x < 0.5 { PN_1 } else { -PN_1 },
        _ => {
            eprintln!("Unknown initial conditions id {}", id);
            |_| PN_0
        }
    }
}

pub struct PipeModel {
    pub time: f64,
    pub len: f64,
    pub n: usize,
    pub sigma: f64,
    pub a: f64,
    pub rho: f64,
    rho_a: f64,
    h: f64,
    h2: f64,
    tau: f64,
    tau_h: f64,
    pub bl: BoundaryCondition,
    pub br: BoundaryCondition,
    pub x: Vec<f64>,
    x2: Vec<f64>,
    pub u1: Vec<f64>,
    pub p1: Vec<f64>,
    u: Vec<f64>,
    p: Vec<f64>,
    pub un_id: String,
    pub pn_id: String,
}

impl PipeModel {
    pub fn new(len: f64, n: usize) -> Self {
        Self {
            time: 0.0,
            len,
            n,
            sigma: DEFAULT_SIGMA,
            a: DEFAULT_A,
            rho: DEFAULT_RHO,
            rho_a: 0.0,
            h: 0.0,
            h2: 0.0,
            tau: 0.0,
            tau_h: 0.0,
            bl: DEFAULT_CONDITION_L,
            br: DEFAULT_CONDITION_R,
            x: vec![],
            x2: vec![],
            u1: vec![],
            p1: vec![],
            u: vec![],
            p: vec![],
            un_id: "▄▄▄▄▄▄".to_string(),
            pn_id: "██▄▄▄▄".to_string(),
        }
    }

    // Reset the simulation
    pub fn reset(&mut self) {
        self.time = 0.0;

        self.h = self.len / (self.n as f64);
        self.h2 = self.h / 2.0;
        self.tau = self.sigma * self.h / self.a;
        self.rho_a = self.rho * self.a;
        self.tau_h = self.tau / self.h;

        self.x = (0..self.n + 1).map(|i| self.h * (i as f64)).collect();
        self.x2 = self
            .x
            .split_last()
            .unwrap()
            .1
            .into_iter()
            .map(|x| x + self.h2)
            .collect();

        let initial_u = f_initial_u(&self.un_id);
        let initial_p = f_initial_p(&self.pn_id);

        self.u1 = self
            .x2
            .clone()
            .into_iter()
            .map(|x| initial_u(x / self.len))
            .collect();
        self.p1 = self
            .x2
            .clone()
            .into_iter()
            .map(|x| initial_p(x / self.len))
            .collect();

        self.u = vec![0.0; self.n + 1];
        self.p = vec![0.0; self.n + 1];
    }

    // Perform single step of the simulation
    pub fn step(&mut self) {
        self.time = self.time + self.tau;

        // Solve system of 2 equations with 2 variables
        fn simq2(a: [[f64; 2]; 2], b: [f64; 2]) -> Option<[f64; 2]> {
            let delta = a[0][0] * a[1][1] - a[1][0] * a[0][1];
            if delta == 0.0 {
                return None;
            } else {
                Some([
                    (b[0] * a[1][1] - b[1] * a[0][1]) / delta,
                    (b[1] * a[0][0] - b[0] * a[1][0]) / delta,
                ])
            }
        }

        // Calculate conditions on the left tip
        let s = [[1.0, -1.0 / self.rho_a], [self.bl.c, self.bl.b]];
        let y = [self.u1[0] - self.p1[0] / self.rho_a, 0.0];
        let w = simq2(s, y).unwrap();
        self.u[0] = w[0];
        self.p[0] = w[1];
        // Alternative without solving a system of equations:
        // self.u[0] = self.bl.b * (-self.p1[0] + self.rho_a * self.u1[0]) / (self.bl.c + self.rho_a * self.bl.b);
        // self.p[0] = self.bl.c * (self.p1[0] - self.rho_a * self.u1[0]) / (self.bl.c + self.rho_a * self.bl.b);

        // Calculate conditions on the right tip
        let s = [[1.0, 1.0 / self.rho_a], [self.br.c, self.br.b]];
        let y = [self.u1[self.n - 1] + self.p1[self.n - 1] / self.rho_a, 0.0];
        let w = simq2(s, y).unwrap();
        self.u[self.n] = w[0];
        self.p[self.n] = w[1];

        // Alternative without solving a system of equations:
        // self.u[self.n] = self.br.b * (-self.p1[self.n-1] + self.rho_a * self.u1[self.n-1]) / (-self.br.c + self.rho_a * self.br.b);
        // self.p[self.n] = self.br.c * (self.p1[self.n-1] - self.rho_a * self.u1[self.n-1]) / (self.br.c - self.rho_a * self.br.b);

        for i in 1..self.n {
            self.u[i] =
                ((self.u1[i] + self.u1[i - 1]) - (self.p1[i] - self.p1[i - 1]) / self.rho_a) / 2.0;
            self.p[i] =
                ((self.p1[i] + self.p1[i - 1]) - (self.u1[i] - self.u1[i - 1]) * self.rho_a) / 2.0;
        }

        for i in 0..self.n {
            self.u1[i] = self.u1[i] - (self.p[i + 1] - self.p[i]) * self.tau_h / self.rho;
            self.p1[i] =
                self.p1[i] - (self.u[i + 1] - self.u[i]) * self.rho_a * self.tau_h * self.a;
        }
    }

    // Set ID for initial conditions for velocities function
    pub fn set_initial_u(&mut self, id: &str) {
        self.un_id = id.to_string();
    }

    // Set ID for initial conditions for pressures function
    pub fn set_initial_p(&mut self, id: &str) {
        self.pn_id = id.to_string();
    }
}
