use fltk::{*, prelude::{*}};

use std::cell::RefCell;
use std::rc::Rc;

use crate::wave_model::WaveModel;
use crate::frame_saver::FrameSaver;

#[derive(Copy, Clone)]
struct Area {
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
}

const MARGIN: i32 = 40;
const TICK_SIZE: i32 = 15;
const TICKS_COUNT_X: i32 = 20;
const TICKS_COUNT_Y: i32 = 30;

pub struct WaveWidget {
    inner: widget::Widget,
    area: Area,
    pixel_x: f64,
    pixel_y: f64,
    offs: Rc<RefCell<draw::Offscreen>>,
    frame_saver: FrameSaver,
}

impl WaveWidget {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        let mut inner = widget::Widget::default()
            .with_size(width, height)
            .with_pos(x, y);

        let area = Area{ xmin: -1.0, xmax: 1.0, ymin: -1.0, ymax: 0.5 };
        let pixel_x = (area.xmax - area.xmin) / (width - MARGIN * 2 - TICK_SIZE) as f64;
        let pixel_y = (area.ymax - area.ymin) / (height - MARGIN * 2 - TICK_SIZE) as f64;

        let offs = draw::Offscreen::new(width, height).unwrap();

        // Initial cleanup
        offs.begin();
        draw::draw_rect_fill(0, 0, width, height, enums::Color::White);

        draw::set_line_style(draw::LineStyle::Solid, 1);
        draw::set_draw_color(enums::Color::Black);
        draw::draw_rect(0, 0, width, height);

        draw::set_line_style(draw::LineStyle::Solid, 0);
        offs.end();
        
        let offs = Rc::from(RefCell::from(offs));
        inner.draw({
            let offs = offs.clone();
            move |i| {
                let offs = offs.borrow_mut();
                offs.copy(i.x(), i.y(), i.w(), i.h(), 0, 0);
            }
        });

        inner.handle(move |_i, ev| match ev {
            _ => false,
        });

        let frame_saver = FrameSaver::new();

        Self {
            inner,
            area,
            pixel_x,
            pixel_y,
            offs,
            frame_saver,
        }
    }

    fn get_x(&self, x: f64) -> f64 {
        (x - self.area.xmin) / self.pixel_x + (MARGIN + TICK_SIZE) as f64
    }

    fn get_y(&self, y: f64) -> f64 {
        (self.area.ymax - y) / self.pixel_y + MARGIN as f64
    }

    pub fn draw_model(&mut self, m: &WaveModel) {
        self.offs.borrow().begin();

        let (width, height) = (self.w(), self.h());
        let area = self.area;

        let scale_x = (area.xmax - area.xmin) / m.delta;
        let scale_z = (AXIS_Y - area.ymin) / m.h;

        // Actual vertical ranges based on model data
        let y_min = -m.h;
        let y_max = m.h * area.ymax.abs() / area.ymin.abs();

        // Clear screen
        draw::draw_rect_fill(0, 0, width, height, enums::Color::White);

        const TEXT_COLOR: enums::Color = enums::Color::Black;

        draw::set_draw_color(TEXT_COLOR);
        draw::set_font(enums::Font::Helvetica, 16);

        let time_str = format!("time: {:.4}", m.time);
        draw::draw_text2(&time_str,
            self.get_x((area.xmax - area.xmin) / 2.0 + area.xmin) as i32, self.get_y(area.ymin) as i32 + TICK_SIZE,
            0, 0, enums::Align::Top);


        // Draw gradient heatmap (lowermost)
        fn get_gradient(c1: enums::Color, c2: enums::Color, n: usize) -> Vec<enums::Color> {
            let (r1, g1, b1) = c1.to_rgb();
            let (r2, g2, b2) = c2.to_rgb();
            let mut colors: Vec<enums::Color> = Vec::new();
            colors.reserve(n);
            for i in 0..n {
                let x: f32 = (i as f32) / (n - 1) as f32;
                let r: f32 = (r2 as f32 - r1 as f32) * x + r1 as f32;
                let g: f32 = (g2 as f32 - g1 as f32) * x + g1 as f32;
                let b: f32 = (b2 as f32 - b1 as f32) * x + b1 as f32;
                let c = enums::Color::from_rgb(r as u8, g as u8, b as u8);
                colors.push(c);
            }
            colors
        }

        const GRADIENT_COLOR_A: enums::Color = enums::Color::from_u32(0xf7fbff);
        const GRADIENT_COLOR_B: enums::Color = enums::Color::from_u32(0x08306b);
        let gradient = get_gradient(GRADIENT_COLOR_A, GRADIENT_COLOR_B, m.zn);

        for j in 0..m.zn-1 {
            draw::begin_complex_polygon();
            let color = gradient[j];
            draw::set_draw_color(color);
            for i in 0..m.xn {
                let idx = i * m.zn + j;
                let p = m.points[idx];
                draw::vertex(self.get_x(p.x * scale_x + area.xmin), 
                    self.get_y((m.h + p.z) * scale_z + area.ymin));
            }
            for i in (0..m.xn).rev() {
                let idx = i * m.zn + j + 1;
                let p = m.points[idx];
                draw::vertex(self.get_x(p.x * scale_x + area.xmin), 
                    self.get_y((m.h + p.z) * scale_z + area.ymin));
            }
            draw::end_complex_polygon();
        }

        // Draw bounding box
        const BOX_COLOR: enums::Color = enums::Color::Black;

        draw::set_line_style(draw::LineStyle::Solid, 1);
        draw::set_draw_color(BOX_COLOR);
        draw::begin_loop();
        draw::vertex(self.get_x(area.xmin), self.get_y(area.ymin));
        draw::vertex(self.get_x(area.xmin), self.get_y(area.ymax));
        draw::vertex(self.get_x(area.xmax), self.get_y(area.ymax));
        draw::vertex(self.get_x(area.xmax), self.get_y(area.ymin));
        draw::end_loop();

        // Ticks
        let dx = (area.xmax - area.xmin) / TICKS_COUNT_X as f64;
        for i in 0..TICKS_COUNT_X+1 {
            draw::draw_yxline(
                self.get_x(area.xmin + dx * (i as f64)) as i32,
                self.get_y(area.ymin) as i32,
                self.get_y(area.ymin) as i32 + TICK_SIZE / (if i % 2 == 0 { 1 } else { 2 }));
        }

        let dy = (area.ymax - area.ymin) / TICKS_COUNT_Y as f64;
        for i in 0..TICKS_COUNT_Y+1 {
            draw::draw_xyline(
                self.get_x(area.xmin) as i32,
                self.get_y(area.ymin + dy * (i as f64)) as i32,
                self.get_x(area.xmin) as i32 - TICK_SIZE / (if i % 2 == 0 { 1 } else { 2 }));
        }

        // Draw range labels
        draw::set_draw_color(TEXT_COLOR);
        draw::set_font(enums::Font::Helvetica, 14);

        let xmin_str = format!("{:.2}", area.xmin);
        draw::draw_text2(&xmin_str,
            self.get_x(area.xmin) as i32, self.get_y(area.ymin) as i32 + TICK_SIZE,
            0, 0, enums::Align::TopLeft);

        let xmax_str = format!("{:.2}", area.xmax);
        draw::draw_text2(&xmax_str,
            self.get_x(area.xmax) as i32, self.get_y(area.ymin) as i32 + TICK_SIZE,
            0, 0, enums::Align::TopRight);

        let ymin_str = format!("{:.2}", y_min);
        draw::draw_text2(&ymin_str,
            self.get_x(area.xmin) as i32 - TICK_SIZE - 2, self.get_y(area.ymin) as i32,
            0, 0, enums::Align::Right);

        let ymax_str = format!("{:.2}", y_max);
        draw::draw_text2(&ymax_str,
            self.get_x(area.xmin) as i32 - TICK_SIZE - 2, self.get_y(area.ymax) as i32,
            0, 0, enums::Align::Right);

        // Draw axes
        const AXIS_Y: f64 = 0.0;
        const AXES_COLOR: enums::Color = enums::Color::Black;

        draw::set_line_style(draw::LineStyle::DashDot, 1);
        draw::set_draw_color(AXES_COLOR);
        draw::draw_line(self.get_x(area.xmin) as i32, self.get_y(AXIS_Y) as i32,
            self.get_x(area.xmax) as i32, self.get_y(AXIS_Y) as i32);

        draw::set_line_style(draw::LineStyle::Solid, 0);

        draw::set_draw_color(TEXT_COLOR);
        draw::set_font(enums::Font::Helvetica, 14);

        let yaxis_str = format!("{:.2}", AXIS_Y);
        draw::draw_text2(&yaxis_str,
            self.get_x(area.xmin) as i32 - TICK_SIZE - 2, self.get_y(AXIS_Y) as i32,
            0, 0, enums::Align::Right);

        // Draw model
        const MODEL_LINES_COLOR: enums::Color = enums::Color::Black;
        const MODEL_POINTS_COLOR: enums::Color = enums::Color::Black;

        draw::set_line_style(draw::LineStyle::Solid, 1);
        draw::set_draw_color(MODEL_LINES_COLOR);

        for j in 0..m.zn {
            draw::begin_line();
            for i in 0..m.xn {
                let idx: usize = i * m.zn + j;
                let p = m.points[idx];
                draw::vertex(self.get_x(p.x * scale_x + area.xmin), 
                    self.get_y((m.h + p.z) * scale_z + area.ymin));
            }
            draw::end_line();
        }

        for j in 0..m.zn {
            for i in 0..m.xn {
                let idx: usize = i * m.zn + j;
                let p = m.points[idx];
                let x: i32 = self.get_x(p.x * scale_x + area.xmin) as i32;
                let y: i32 = self.get_y((m.h + p.z) * scale_z + area.ymin) as i32;

                const POINT_SIZE: i32 = 2;
                draw::draw_rect_fill(x - POINT_SIZE, y - POINT_SIZE, POINT_SIZE*2, POINT_SIZE*2,
                    MODEL_POINTS_COLOR);
            }
        }

        self.offs.borrow().end();

        self.redraw();
    }

    pub fn reset_frame_counter(&mut self) {
        self.frame_saver.reset();
    }

    pub fn save_frame(&mut self) {
        match draw::capture_offscreen(&mut self.offs.borrow_mut(), self.w(), self.h()) {
            Ok(img) => {
                let data = img.to_rgb_data();
                self.frame_saver.save_frame(&data, img.width(), img.height());
            }
            Err(error) => { eprintln!("Cannot capture frame to image. Error: {}", error); }
        }
    }
}

widget_extends!(WaveWidget, widget::Widget, inner);
