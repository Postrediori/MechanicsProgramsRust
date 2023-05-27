use fltk::{enums, draw};

use crate::param_list::*;
use crate::draw_primitives::*;
use crate::pendulum_model::*;

// Model of a simple pendulum
const THETA_0: f64 = 45.0;
const LENGTH: f64 = 1.0;
const G: f64 = 9.81;
const DT: f64 = 0.05;

pub struct SimplePendulumModel {
    pub params: ParamList,
    time: f64,
    theta: f64,
    theta_v: f64,
    theta_a: f64,
    length: f64,
}

impl SimplePendulumModel {
    pub fn new() -> Self {
        let params = ParamList::from([
            ("theta0", THETA_0, "Initial pendulum angle"),
            ("g", G, "Gravitational constant"),
            ("dtime", DT, "Time step delta"),
        ]);

        Self {
            params,
            time: 0.0,
            theta: 0.0,
            theta_v: 0.0,
            theta_a: 0.0,
            length: LENGTH,
        }
    }

    fn theta0(&self) -> f64 {
        self.params.get_by_key("theta0")
    }

    fn g(&self) -> f64 {
        self.params.get_by_key("g")
    }

    fn dtime(&self) -> f64 {
        self.params.get_by_key("dtime")
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
        self.theta = self.theta0().to_radians();
        self.theta_v = 0.0;
        self.theta_a = 0.0;
    }

    fn step(&mut self) {
        self.time += self.dtime();
        self.theta_a = -self.g() * self.theta.sin() / self.length;
        self.theta_v += self.theta_a * self.dtime();
        self.theta += self.theta_v * self.dtime();
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
        let l: i32 = h / 2;
    
        // Coordinates of the weight
        let angle: f64 = (90 as f64).to_radians() - self.theta;
        let x1: i32 = (x0 as f64 + (l as f64) * (angle).cos()) as i32;
        let y1: i32 = (y0 as f64 + (l as f64) * (angle).sin()) as i32;
    
        // Draw vertical axis
        draw_axis(x0, y0,
            x0, y0 + ((l as f64) * 1.25) as i32);

        // Draw rest
        const FIX_WIDTH: i32 = 90;
        const FIX_HEIGHT: i32 = 25;
        draw_rest(x0, y0 - FIX_HEIGHT/2, FIX_WIDTH, FIX_HEIGHT);

        // Draw cord
        draw_cord(x0, y0, x1, y1);

        // Draw weight
        draw_weight(x1, y1);

        // Correct finish of drawing
        draw::set_line_style(draw::LineStyle::Solid, 0);

        offs.end();
    }
}
