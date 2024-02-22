use fltk::{enums, draw};

use crate::param_list::*;
use crate::draw_primitives::*;
use crate::pendulum_model::*;

const THETA1_0: f64 = 30.0;
const THETA2_0: f64 = 45.0;
const LENGTH: f64 = 1.0;
const MASS: f64 = 1.0;
const G: f64 = 9.81;
const DT: f64 = 0.05;

fn pendulum_f1(omega1: f64) -> f64 {
    omega1
}

fn pendulum_f2(theta1: f64, omega1: f64, theta2: f64, omega2: f64) -> f64 {
    let t21 = theta2 - theta1;
    let t21s = t21.sin();
    let a = MASS * t21s * (LENGTH * omega2.powi(2) + theta2.sin()) + MASS * (2.0 * t21).sin() * omega1.powi(2) / 2.0 - theta1.sin();
    let b = 1.0 + MASS * t21s.powi(2);
    a / b
}

fn pendulum_f3(omega2: f64) -> f64 {
    omega2
}

fn pendulum_f4(theta1: f64, omega1: f64, theta2: f64, omega2: f64) -> f64 {
    let t21 = theta2 - theta1;
    -(theta2.sin() + t21.sin() * omega1.powi(2) + t21.cos() * pendulum_f2(theta1, omega1, theta2, omega2)) / LENGTH
}

fn pendulum_f(theta1: f64, omega1: f64, theta2: f64, omega2: f64) -> (f64, f64, f64, f64) {
    (pendulum_f1(omega1), pendulum_f2(theta1, omega1, theta2, omega2), pendulum_f3(omega2), pendulum_f4(theta1, omega1, theta2, omega2))
}

pub struct DoublePendulumModel {
    pub params: ParamList,
    time: f64,
    theta1: f64,
    theta2: f64,
    omega1: f64,
    omega2: f64,
}

impl DoublePendulumModel {
    pub fn new() -> Self {
        let params = ParamList::from([
            ("theta1_0", THETA1_0, "Initial angle of first pendulum"),
            ("theta2_0", THETA2_0, "Initial angle of second pendulum"),
            ("g", G, "Gravitational constant"),
            ("dtime", DT, "Time step delta"),
        ]);

        Self {
            params,
            time: 0.0,
            theta1: 0.0,
            theta2: 0.0,
            omega1: 0.0,
            omega2: 0.0,
        }
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
        
        self.theta1 = self.params.get_by_key("theta1_0").to_radians();
        self.theta2 = self.params.get_by_key("theta2_0").to_radians();
    }

    fn step(&mut self) {
        let dtime = self.params.get_by_key("dtime");
        let dtime2 = dtime / 2.0;

        let (k1, l1, m1, n1) = pendulum_f(self.theta1, self.omega1, self.theta2, self.omega2);
        let (k2, l2, m2, n2) = pendulum_f(self.theta1 + k1 * dtime2, self.omega1 + l1 * dtime2,
            self.theta2 + m1 * dtime2, self.omega2 + n1 * dtime2);
        let (k3, l3, m3, n3) = pendulum_f(self.theta1 + k2 * dtime2, self.omega1 + l2 * dtime2,
            self.theta2 + m2 * dtime2, self.omega2 + n2 * dtime2);
        let (k4, l4, m4, n4) = pendulum_f(self.theta1 + k3 * dtime, self.omega1 + l3 * dtime,
            self.theta2 + m3 * dtime, self.omega2 + n3 * dtime);

        self.theta1 += dtime * (k1 + 2.0 * (k2 + k3) + k4) / 6.0;
        self.omega1 += dtime * (l1 + 2.0 * (l2 + l3) + l4) / 6.0;
        self.theta2 += dtime * (m1 + 2.0 * (m2 + m3) + m4) / 6.0;
        self.omega2 += dtime * (n1 + 2.0 * (n2 + n3) + n4) / 6.0;

        self.time += dtime;
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
        let l: i32 = h / 4;

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
        let x1: i32 = (x0 as f64 + (l as f64) * (angle1).cos()) as i32;
        let y1: i32 = (y0 as f64 + (l as f64) * (angle1).sin()) as i32;

        let angle2: f64 = (90 as f64).to_radians() - self.theta2;
        let x2: i32 = (x1 as f64 + (l as f64) * (angle2).cos()) as i32;
        let y2: i32 = (y1 as f64 + (l as f64) * (angle2).sin()) as i32;

        // Draw vertical axis
        draw_axis(x0, y0,
            x0, y0 + ((l as f64) * 2.0) as i32);

        // Draw rest
        const FIX_WIDTH: i32 = 90;
        const FIX_HEIGHT: i32 = 25;
        draw_rest(x0, y0 - FIX_HEIGHT/2, FIX_WIDTH, FIX_HEIGHT);

        // Draw cords
        draw_cord(x0, y0, x1, y1);
        draw_cord(x1, y1, x2, y2);

        // Draw weights
        draw_weight(x1, y1);
        draw_weight(x2, y2);

        draw::set_line_style(draw::LineStyle::Solid, 0);

        offs.end();
    }
}
