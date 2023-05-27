use fltk::{enums, draw};

use crate::param_list::*;
use crate::draw_primitives::*;
use crate::pendulum_model::*;

// Model of an elastic pendulum
const THETA_0: f64 = 45.0;
const X_0: f64 = 0.0;
const LENGTH: f64 = 1.0;
const MASS: f64 = 1.0;
const K: f64 = 30.0;
const G: f64 = 9.81;
const DT: f64 = 0.05;

pub struct ElasticPendulumModel {
    params: ParamList,
    time: f64,
    length: f64,
    mass: f64,
    k: f64,
    theta: f64,
    theta_v: f64,
    theta_a: f64,
    x: f64,
    x_v: f64,
    x_a: f64,
}

impl ElasticPendulumModel {
    pub fn new() -> Self {
        let params = ParamList::from([
            ("theta0", THETA_0, "Initial pendulum angle"),
            ("x0", X_0, "Initial spring stretch"),
            ("g", G, "Gravitational constant"),
            ("dtime", DT, "Time step delta"),
        ]);

        Self {
            params,
            time: 0.0,
            length: LENGTH,
            mass: MASS,
            k: K,
            theta: 0.0,
            theta_v: 0.0,
            theta_a: 0.0,
            x: 0.0,
            x_v: 0.0,
            x_a: 0.0,
        }
    }
}

impl ParametrizedModel for ElasticPendulumModel {}

impl Parametrized for ElasticPendulumModel {
    fn copy_params_from(&mut self, other: &ParamList) {
        self.params.copy_from(other);
    }
    fn get_params(&self) -> ParamList {
        self.params.clone()
    }
}

impl PendulumModel for ElasticPendulumModel {
    fn label(&self) -> &'static str {
        "Elastic Pendulum"
    }

    fn time(&self) -> f64 {
        self.time
    }

    fn restart(&mut self) {
        self.time = 0.0;

        self.theta = self.params.get_by_key("theta0").to_radians();
        self.theta_v = 0.0;
        self.theta_a = 0.0;

        self.x = self.params.get_by_key("x0");
        self.x_v = 0.0;
        self.x_a = 0.0;
    }

    fn step(&mut self) {
        let g = self.params.get_by_key("g");
        let dtime = self.params.get_by_key("dtime");

        self.time += dtime;

        let l = self.length + self.x;

        self.x_a = l * (self.theta_v * self.theta_v) -
            self.k * self.x / self.mass +
            g * self.theta.cos();
        
        self.theta_a = -(g * self.theta.sin() + 2.0 * self.x_v * self.theta_v) / l;

        self.x_v += self.x_a * dtime;
        self.x += self.x_v * dtime;

        self.theta_v += self.theta_a * dtime;
        self.theta += self.theta_v * dtime;
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

        // Draw labels
        draw::set_draw_color(TEXT_COLOR);
        draw::set_font(enums::Font::Helvetica, 16);

        draw::draw_text2(self.label(), w / 2, MARGIN, 0, 0, enums::Align::Center);

        let theta_str = format!("θ = {:.2}°", self.theta.to_degrees());
        draw::draw_text2(&theta_str, w / 2, h - MARGIN * 2, 0, 0, enums::Align::Center);

        let time_str = format!("time = {:.2} s", self.time());
        draw::draw_text2(&time_str, w / 2, h - MARGIN, 0, 0, enums::Align::Center);

        // Coordinates of the pivotal point
        let x0: i32 = w / 2;
        let y0: i32 = h / 4;
        let l0: i32 = h / 3;
        let l: i32 = (l0 as f64 * (1.0 + self.x)) as i32;
    
        // Coordinates of the weight
        let angle: f64 = (90 as f64).to_radians() - self.theta;
        let x1: i32 = (x0 as f64 + (l as f64) * (angle).cos()) as i32;
        let y1: i32 = (y0 as f64 + (l as f64) * (angle).sin()) as i32;
    
        // Draw vertical axis
        draw_axis(x0, y0,
            x0, y0 + ((l0 as f64) * 1.25) as i32);

        // Draw rest
        const FIX_WIDTH: i32 = 90;
        const FIX_HEIGHT: i32 = 25;
        draw_rest(x0, y0 - FIX_HEIGHT/2, FIX_WIDTH, FIX_HEIGHT);

        // Draw spring
        const SPRING_WIDTH: i32 = 15;
        draw_spring(x0, y0, x1, y1,
            8, SPRING_WIDTH);

        // Draw weight
        draw_weight(x1, y1);

        draw::set_line_style(draw::LineStyle::Solid, 0);

        offs.end();
    }
}
