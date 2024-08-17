use fltk::{draw, enums};

use crate::draw_primitives::*;
use crate::param_list::*;
use crate::pendulum_model::*;

const THETA1_0: f64 = 30.0;
const THETA2_0: f64 = 45.0;
const LENGTH: f64 = 1.0;
const MASS: f64 = 1.0;
const DT: f64 = 0.05;

pub struct DoublePendulumModel {
    pub params: ParamList,
    time: f64,
    dtime: f64,
    dtime2: f64,
    length: f64,
    mass: f64,
    theta1: f64,
    theta2: f64,
    omega1: f64,
    omega2: f64,
}

impl DoublePendulumModel {
    pub fn new() -> Self {
        let params = ParamList::from([
            (
                "theta1_0",
                "θ1(0)",
                THETA1_0,
                "Initial angle of first pendulum",
            ),
            (
                "theta2_0",
                "θ2(0)",
                THETA2_0,
                "Initial angle of second pendulum",
            ),
            ("L", "L", LENGTH, "Length of each pendulum"),
            ("mass", "m", MASS, "Mass of each pendulum"),
            ("dtime", "ΔT", DT, "Time step delta"),
        ]);

        Self {
            params,
            time: 0.0,
            dtime: DT,
            dtime2: DT / 2.0,
            length: LENGTH,
            mass: MASS,
            theta1: 0.0,
            theta2: 0.0,
            omega1: 0.0,
            omega2: 0.0,
        }
    }

    fn f1(omega1: f64) -> f64 {
        omega1
    }

    fn f2(&self, theta1: f64, omega1: f64, theta2: f64, omega2: f64) -> f64 {
        let t21 = theta2 - theta1;
        let t21s = t21.sin();
        let a = self.mass * t21s * (LENGTH * omega2.powi(2) + theta2.cos())
            + self.mass * (2.0 * t21).sin() * omega1.powi(2) / 2.0
            - theta1.sin();
        let b = 1.0 + self.mass * t21s.powi(2);
        a / b
    }

    fn f3(omega2: f64) -> f64 {
        omega2
    }

    fn f4(&self, theta1: f64, omega1: f64, theta2: f64, omega2: f64) -> f64 {
        let t21 = theta2 - theta1;
        -(theta2.sin()
            + t21.sin() * omega1.powi(2)
            + t21.cos() * self.f2(theta1, omega1, theta2, omega2))
            / self.length
    }

    fn f(&self, theta1: f64, omega1: f64, theta2: f64, omega2: f64) -> (f64, f64, f64, f64) {
        (
            DoublePendulumModel::f1(omega1),
            self.f2(theta1, omega1, theta2, omega2),
            DoublePendulumModel::f3(omega2),
            self.f4(theta1, omega1, theta2, omega2),
        )
    }
}

impl ParametrizedModel for DoublePendulumModel {}

impl Parametrized for DoublePendulumModel {
    fn copy_params_from(&mut self, other: &ParamList) {
        self.params.copy_from(other);
    }
    fn get_params(&self) -> ParamList {
        self.params.clone()
    }
}

impl PendulumModel for DoublePendulumModel {
    fn label(&self) -> &'static str {
        "Double pendulum"
    }

    fn time(&self) -> f64 {
        self.time
    }

    fn restart(&mut self) {
        self.time = 0.0;
        self.dtime = self.params.get_by_key("dtime");
        self.dtime2 = self.dtime / 2.0;

        self.length = self.params.get_by_key("L");
        self.mass = self.params.get_by_key("mass");

        self.theta1 = self.params.get_by_key("theta1_0").to_radians();
        self.theta2 = self.params.get_by_key("theta2_0").to_radians();
    }

    fn step(&mut self) {
        let (k1, l1, m1, n1) = self.f(self.theta1, self.omega1, self.theta2, self.omega2);
        let (k2, l2, m2, n2) = self.f(
            self.theta1 + k1 * self.dtime2,
            self.omega1 + l1 * self.dtime2,
            self.theta2 + m1 * self.dtime2,
            self.omega2 + n1 * self.dtime2,
        );
        let (k3, l3, m3, n3) = self.f(
            self.theta1 + k2 * self.dtime2,
            self.omega1 + l2 * self.dtime2,
            self.theta2 + m2 * self.dtime2,
            self.omega2 + n2 * self.dtime2,
        );
        let (k4, l4, m4, n4) = self.f(
            self.theta1 + k3 * self.dtime,
            self.omega1 + l3 * self.dtime,
            self.theta2 + m3 * self.dtime,
            self.omega2 + n3 * self.dtime,
        );

        self.theta1 += self.dtime * (k1 + 2.0 * (k2 + k3) + k4) / 6.0;
        self.omega1 += self.dtime * (l1 + 2.0 * (l2 + l3) + l4) / 6.0;
        self.theta2 += self.dtime * (m1 + 2.0 * (m2 + m3) + m4) / 6.0;
        self.omega2 += self.dtime * (n1 + 2.0 * (n2 + n3) + n4) / 6.0;

        self.time += self.dtime;
    }

    fn draw(&self, w: i32, h: i32, offs: &draw::Offscreen) {
        offs.begin();

        // Geometry sizes
        const MARGIN: i32 = 20;

        // Color palette
        const BG_COLOR: enums::Color = enums::Color::White;
        const BOUNDS_COLOR: enums::Color = enums::Color::Dark3;
        const TEXT_COLOR: enums::Color = enums::Color::Black;

        // Clear background
        draw::draw_rect_fill(0, 0, w, h, BG_COLOR);

        // Draw bounds
        draw::set_draw_color(BOUNDS_COLOR);
        draw::set_line_style(draw::LineStyle::Solid, 1);
        draw::draw_rect(0, 0, w, h);

        // Coordinates of the pivotal points
        let x0: i32 = w / 2;
        let y0: i32 = h / 4;
        let l = self.length * (h / 4) as f64;

        // Draw labels
        draw::set_draw_color(TEXT_COLOR);
        draw::set_font(enums::Font::Helvetica, 16);

        draw::draw_text2(self.label(), w / 2, MARGIN, 0, 0, enums::Align::Center);

        let theta1_str = format!("θ1 = {:.2}°", self.theta1.to_degrees() % 360.0);
        draw::draw_text2(&theta1_str, x0, h - MARGIN * 3, 0, 0, enums::Align::Center);

        let theta2_str = format!("θ2 = {:.2}°", self.theta2.to_degrees() % 360.0);
        draw::draw_text2(&theta2_str, x0, h - MARGIN * 2, 0, 0, enums::Align::Center);

        let time_str = format!("time = {:.2} s", self.time());
        draw::draw_text2(&time_str, w / 2, h - MARGIN, 0, 0, enums::Align::Center);

        // Coordinates of pendulums
        let angle1: f64 = (90 as f64).to_radians() - self.theta1;
        let x1: i32 = (x0 as f64 + l * (angle1).cos()) as i32;
        let y1: i32 = (y0 as f64 + l * (angle1).sin()) as i32;

        let angle2: f64 = (90 as f64).to_radians() - self.theta2;
        let x2: i32 = (x1 as f64 + l * (angle2).cos()) as i32;
        let y2: i32 = (y1 as f64 + l * (angle2).sin()) as i32;

        // Draw vertical axis
        draw_axis(x0, y0, x0, y0 + (l * 2.0) as i32);

        // Draw rest
        const FIX_WIDTH: i32 = 90;
        const FIX_HEIGHT: i32 = 25;
        draw_rest(x0, y0 - FIX_HEIGHT / 2, FIX_WIDTH, FIX_HEIGHT);

        // Draw cords
        draw_cord(x0, y0, x1, y1);
        draw_cord(x1, y1, x2, y2);

        // Draw weights
        draw_weight(x1, y1);
        draw_weight(x2, y2);

        offs.end();
    }
}
