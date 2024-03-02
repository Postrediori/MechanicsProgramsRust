use fltk::{enums, draw};

use crate::param_list::*;
use crate::draw_primitives::*;
use crate::pendulum_model::*;

const THETA1_0: f64 = 45.0;
const THETA2_0: f64 = 30.0;
const LENGTH: f64 = 1.0;
const MASS: f64 = 1.0;
const K: f64 = 30.0;
const G: f64 = 9.81;
const DT: f64 = 0.05;

pub struct CoupledPendulumsModel {
    pub params: ParamList,
    time: f64,
    dtime: f64,
    length: f64,
    mass: f64,
    k: f64,
    theta1: f64,
    theta2: f64,
    omega1: f64,
    omega2: f64,
    a: f64,
    b: f64,
    g: f64,
}

impl CoupledPendulumsModel {
    pub fn new() -> Self {
        let params = ParamList::from([
            ("theta1_0", "θ1(0)", THETA1_0, "Initial angle of left pendulum"),
            ("theta2_0", "θ2(0)", THETA2_0, "Initial angle of right pendulum"),
            ("L", "L", LENGTH, "Pendulum length"),
            ("mass", "m", MASS, "Mass of each pendulum"),
            ("k", "k", K, "Spring constant"),
            ("g", "g", G, "Gravitational constant"),
            ("dtime", "ΔT", DT, "Time step delta"),
        ]);

        Self {
            params,
            time: 0.0,
            dtime: DT,
            length: LENGTH,
            mass: MASS,
            k: K,
            theta1: 0.0,
            theta2: 0.0,
            omega1: 0.0,
            omega2: 0.0,
            a: 0.0,
            b: 0.0,
            g: G,
        }
    }
}

impl ParametrizedModel for CoupledPendulumsModel {}

impl Parametrized for CoupledPendulumsModel {
    fn copy_params_from(&mut self, other: &ParamList) {
        self.params.copy_from(other);
    }
    fn get_params(&self) -> ParamList {
        self.params.clone()
    }
}

impl PendulumModel for CoupledPendulumsModel {
    fn label(&self) -> &'static str {
        "Coupled pendulums"
    }

    fn time(&self) -> f64 {
        self.time
    }

    fn restart(&mut self) {
        self.time = 0.0;
        self.dtime = self.params.get_by_key("dtime");
        
        self.theta1 = self.params.get_by_key("theta1_0").to_radians();
        self.theta2 = self.params.get_by_key("theta2_0").to_radians();

        self.length = self.params.get_by_key("L");
        self.mass = self.params.get_by_key("mass");
        self.k = self.params.get_by_key("k");
        self.g = self.params.get_by_key("g");

        self.omega1 = (self.g / self.length).sqrt();
        self.omega2 = (self.g / self.length + 2.0 * self.k / self.mass).sqrt();

        self.a = self.theta1 + self.theta2;
        self.b = self.theta1 - self.theta2;
    }

    fn step(&mut self) {
        self.time += self.dtime;

        self.theta1 = self.a * (self.omega1 * self.time).cos() / 2.0 +
            self.b * (self.omega2 * self.time).cos() / 2.0;
        self.theta2 = self.a * (self.omega1 * self.time).cos() / 2.0 -
            self.b * (self.omega2 * self.time).cos() / 2.0;
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
        let x0_1: i32 = w / 3;
        let x0_2: i32 = 2 * w / 3;
        let y0: i32 = h / 4;
        let l: f64 = self.length * (h / 3) as f64;

        // Draw labels
        draw::set_draw_color(TEXT_COLOR);
        draw::set_font(enums::Font::Helvetica, 16);

        draw::draw_text2(self.label(), w / 2, MARGIN, 0, 0, enums::Align::Center);

        let theta1_str = format!("θ1 = {:.2}°", self.theta1.to_degrees());
        draw::draw_text2(&theta1_str, x0_1, h - MARGIN * 2, 0, 0, enums::Align::Center);

        let theta2_str = format!("θ2 = {:.2}°", self.theta2.to_degrees());
        draw::draw_text2(&theta2_str, x0_2, h - MARGIN * 2, 0, 0, enums::Align::Center);

        let time_str = format!("time = {:.2} s", self.time());
        draw::draw_text2(&time_str, w / 2, h - MARGIN, 0, 0, enums::Align::Center);
    
        // Coordinates of pendulums
        let angle1: f64 = (90 as f64).to_radians() - self.theta1;
        let x1: i32 = (x0_1 as f64 + l * (angle1).cos()) as i32;
        let y1: i32 = (y0 as f64 + l * (angle1).sin()) as i32;

        let angle2: f64 = (90 as f64).to_radians() - self.theta2;
        let x2: i32 = (x0_2 as f64 + l * (angle2).cos()) as i32;
        let y2: i32 = (y0 as f64 + l * (angle2).sin()) as i32;

        // Draw vertical axis
        draw_axis(x0_1, y0,
            x0_1, y0 + (l * 1.25) as i32);

        draw_axis(x0_2, y0,
            x0_2, y0 + (l * 1.25) as i32);

        // Draw linking spring
        const SPRING_WIDTH: i32 = 10;
        draw_spring(x1, y1, x2, y2, 8, SPRING_WIDTH);

        // Draw rest
        const FIX_WIDTH: i32 = 90;
        const FIX_HEIGHT: i32 = 25;
        draw_rest(x0_1, y0 - FIX_HEIGHT/2, FIX_WIDTH, FIX_HEIGHT);
        draw_rest(x0_2, y0 - FIX_HEIGHT/2, FIX_WIDTH, FIX_HEIGHT);

        // Draw cords
        draw_cord(x0_1, y0, x1, y1);
        draw_cord(x0_2, y0, x2, y2);

        // Draw weights
        draw_weight(x1, y1);
        draw_weight(x2, y2);

        draw::set_line_style(draw::LineStyle::Solid, 0);

        offs.end();
    }
}
