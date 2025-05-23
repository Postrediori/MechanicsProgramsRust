#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_lossless)]

use fltk::{draw, enums};

use crate::draw_primitives::{draw_axis, draw_cord, draw_rest, draw_weight};
use crate::param_list::{ParamList, Parametrized};
use crate::pendulum_model::{ParametrizedModel, PendulumModel};

// Model of a simple pendulum
const THETA_0: f64 = 45.0;
const LENGTH: f64 = 1.0;
const G: f64 = 9.81;
const DT: f64 = 0.05;

pub struct SimplePendulumModel {
    pub params: ParamList,
    time: f64,
    dtime: f64,
    theta: f64,
    theta_v: f64,
    theta_a: f64,
    length: f64,
    g: f64,
}

impl SimplePendulumModel {
    pub fn new() -> Self {
        let params = ParamList::from([
            ("theta0", "θ(0)", THETA_0, "Initial pendulum angle"),
            ("L", "L", LENGTH, "Pendulum length"),
            ("g", "g", G, "Gravitational constant"),
            ("dtime", "ΔT", DT, "Time step delta"),
        ]);

        Self {
            params,
            time: 0.0,
            dtime: DT,
            theta: 0.0,
            theta_v: 0.0,
            theta_a: 0.0,
            length: LENGTH,
            g: G,
        }
    }
}

impl ParametrizedModel for SimplePendulumModel {}

impl Parametrized for SimplePendulumModel {
    fn copy_params_from(&mut self, other: &ParamList) {
        self.params.copy_from(other);
    }
    fn get_params(&self) -> ParamList {
        self.params.clone()
    }
}

impl PendulumModel for SimplePendulumModel {
    fn label(&self) -> &'static str {
        "Simple Pendulum"
    }

    fn time(&self) -> f64 {
        self.time
    }

    fn restart(&mut self) {
        self.time = 0.0;
        self.dtime = self.params.get_by_key("dtime");
        self.theta = self.params.get_by_key("theta0").to_radians();
        self.theta_v = 0.0;
        self.theta_a = 0.0;
        self.length = self.params.get_by_key("L");
        self.g = self.params.get_by_key("g");
    }

    fn step(&mut self) {
        self.time += self.dtime;
        self.theta_a = -self.g * self.theta.sin() / self.length;
        self.theta_v += self.theta_a * self.dtime;
        self.theta += self.theta_v * self.dtime;
    }

    fn draw(&self, w: i32, h: i32, offs: &draw::Offscreen) {
        // Geometry sizes
        const MARGIN: i32 = 20;
        const FIX_WIDTH: i32 = 90;
        const FIX_HEIGHT: i32 = 25;

        // Color palette
        const BG_COLOR: enums::Color = enums::Color::White;
        const BOUNDS_COLOR: enums::Color = enums::Color::Dark3;
        const TEXT_COLOR: enums::Color = enums::Color::Black;

        offs.begin();

        // Clear background
        draw::draw_rect_fill(0, 0, w, h, BG_COLOR);

        // Draw bounds
        draw::set_draw_color(BOUNDS_COLOR);
        draw::set_line_style(draw::LineStyle::Solid, 1);
        draw::draw_rect(0, 0, w, h);

        // Draw labels
        draw::set_draw_color(TEXT_COLOR);
        draw::set_font(enums::Font::Helvetica, 16);

        draw::draw_text2(self.label(), w / 2, MARGIN, 0, 0, enums::Align::Center);

        let theta_str = format!("θ = {:.2}°", self.theta.to_degrees());
        draw::draw_text2(
            &theta_str,
            w / 2,
            h - MARGIN * 2,
            0,
            0,
            enums::Align::Center,
        );

        let time_str = format!("time = {:.2} s", self.time());
        draw::draw_text2(&time_str, w / 2, h - MARGIN, 0, 0, enums::Align::Center);

        // Coordinates of the pivotal point
        let x0: i32 = w / 2;
        let y0: i32 = h / 4;
        let l: f64 = self.length * ((h / 2) as f64);

        // Coordinates of the weight
        let angle: f64 = 90_f64.to_radians() - self.theta;
        let x1: i32 = (x0 as f64 + l * (angle).cos()) as i32;
        let y1: i32 = (y0 as f64 + l * (angle).sin()) as i32;

        // Draw vertical axis
        draw_axis(x0, y0, x0, y0 + (l * 1.25) as i32);

        // Draw rest
        draw_rest(x0, y0 - FIX_HEIGHT / 2, FIX_WIDTH, FIX_HEIGHT);

        // Draw cord
        draw_cord(x0, y0, x1, y1);

        // Draw weight
        draw_weight(x1, y1);

        // Correct finish of drawing
        draw::set_line_style(draw::LineStyle::Solid, 0);

        offs.end();
    }
}
