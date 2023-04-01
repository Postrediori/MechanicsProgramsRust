use fltk::{*, prelude::*};

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Copy, Clone)]
struct Area {
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
}

const MARGIN: i32 = 25;
const TICK_SIZE: i32 = 15;
const TICKS_COUNT: i32 = 20;

pub struct PlotWidget {
    inner: widget::Widget,
    area: Area,
    pixel_x: f64,
    pixel_y: f64,
    offs: Rc<RefCell<draw::Offscreen>>,
}

impl PlotWidget {
    pub fn new(x: i32, y: i32, width: i32, height: i32, title: &str) -> Self {
        let mut inner = widget::Widget::default()
            .with_size(width, height)
            .with_pos(x, y)
            .with_label(title);

        let offs = draw::Offscreen::new(width, height).unwrap();

        let offs = Rc::from(RefCell::from(offs));

        inner.draw({
            let offs = offs.clone();
            move |i| {
                let offs = offs.borrow();
                offs.copy(i.x(), i.y(), i.w(), i.h(), 0, 0);
            }
        });

        Self {
            inner,
            area: Area { xmin: 0.0, xmax: 0.0, ymin: 0.0, ymax: 0.0 },
            pixel_x: 0.0,
            pixel_y: 0.0,
            offs
        }
    }

    fn get_x(&self, x: f64) -> f64 {
        (x - self.area.xmin) / self.pixel_x + (MARGIN + TICK_SIZE) as f64
    }

    fn get_y(&self, y: f64) -> f64 {
        (self.area.ymax - y) / self.pixel_y + MARGIN as f64
    }

    pub fn draw_plot(&mut self, x_points: &Vec<f64>, y_points: &Vec<f64>, len: f64) {
        let (width, height) = (self.w(), self.h());
       
        let area = Area { xmin: 0.0, xmax: len, ymin: -2.0, ymax: 2.0 };
        self.area = area;

        self.pixel_x = (area.xmax - area.xmin) / (self.w() - (MARGIN * 2 + TICK_SIZE)) as f64;
        self.pixel_y = (area.ymax - area.ymin) / (self.h() - (MARGIN * 2 + TICK_SIZE)) as f64;

        self.offs.borrow().begin();

        // Initial cleanup
        draw::draw_rect_fill(0, 0, width, height, enums::Color::White);

        // Title
        draw::set_draw_color(enums::Color::Black);
        draw::set_font(enums::Font::Helvetica, 16);

        let title = self.label();
        draw::draw_text2(&title, self.w() / 2, MARGIN / 2, 0, 0, enums::Align::Center);

        // Heatmap (lowermost layer)
        const AXIS_Y: f64 = 0.0;

        const PALETTE: [enums::Color; 11] = [
            enums::Color::from_u32(0xa50027),
            enums::Color::from_u32(0xd73027),
            enums::Color::from_u32(0xf46d43),
            enums::Color::from_u32(0xfdae61),
            enums::Color::from_u32(0xfee090),
            enums::Color::from_u32(0xffffbf),
            enums::Color::from_u32(0xe0f3f9),
            enums::Color::from_u32(0xabd9ea),
            enums::Color::from_u32(0x74add1),
            enums::Color::from_u32(0x4575b4),
            enums::Color::from_u32(0x313695)
        ];
        
        for i in 0..y_points.len() {
            let y = y_points[i];
            let t = 1.0 - (y - area.ymin) / (area.ymax - area.ymin);
            let k = (PALETTE.len() as f64 * t) as usize % PALETTE.len();

            draw::set_draw_color(PALETTE[k]);
            draw::draw_polygon3(
                draw::Coord::<i32>(self.get_x(x_points[i]) as i32, self.get_y(AXIS_Y) as i32),
                draw::Coord::<i32>(self.get_x(x_points[i + 1]) as i32, self.get_y(AXIS_Y) as i32),
                draw::Coord::<i32>(self.get_x(x_points[i + 1]) as i32, self.get_y(y) as i32),
                draw::Coord::<i32>(self.get_x(x_points[i]) as i32, self.get_y(y) as i32));
        }

        // Bounding box
        draw::set_line_style(draw::LineStyle::Solid, 1);
        draw::set_draw_color(enums::Color::Black);
        draw::begin_loop();
        draw::vertex(self.get_x(area.xmin), self.get_y(area.ymin));
        draw::vertex(self.get_x(area.xmin), self.get_y(area.ymax));
        draw::vertex(self.get_x(area.xmax), self.get_y(area.ymax));
        draw::vertex(self.get_x(area.xmax), self.get_y(area.ymin));
        draw::end_loop();

        // Ticks
        let dx = (area.xmax - area.xmin) / TICKS_COUNT as f64;
        for i in 0..TICKS_COUNT+1 {
            draw::draw_yxline(
                self.get_x(area.xmin + dx * (i as f64)) as i32,
                self.get_y(area.ymin) as i32,
                self.get_y(area.ymin) as i32 + TICK_SIZE / (if i % 2 == 0 { 1 } else { 2 }));
        }

        let dy = (area.ymax - area.ymin) / TICKS_COUNT as f64;
        for i in 0..TICKS_COUNT+1 {
            draw::draw_xyline(
                self.get_x(area.xmin) as i32,
                self.get_y(area.ymin + dy * (i as f64)) as i32,
                self.get_x(area.xmin) as i32 - TICK_SIZE / (if i % 2 == 0 { 1 } else { 2 }));
        }

        // Plot ranges
        draw::set_draw_color(enums::Color::Black);
        draw::set_font(enums::Font::Helvetica, 12);

        let xmin_str = format!("{:.1}", area.xmin);
        draw::draw_text2(&xmin_str,
            self.get_x(area.xmin) as i32, self.get_y(area.ymin) as i32 + TICK_SIZE + 2,
            0, 0, enums::Align::TopLeft);

        let xmax_str = format!("{:.1}", area.xmax);
        draw::draw_text2(&xmax_str,
            self.get_x(area.xmax) as i32, self.get_y(area.ymin) as i32 + TICK_SIZE + 2,
            0, 0, enums::Align::TopRight);

        let ymin_str = format!("{:.1}", area.ymin);
        draw::draw_text2(&ymin_str,
            self.get_x(area.xmin) as i32 - TICK_SIZE - 2, self.get_y(area.ymin) as i32,
            0, 0, enums::Align::BottomRight);

        let ymax_str = format!("{:.1}", area.ymax);
        draw::draw_text2(&ymax_str,
            self.get_x(area.xmin) as i32 - TICK_SIZE - 2, self.get_y(area.ymax) as i32,
            0, 0, enums::Align::TopRight);

        // Axis
        draw::set_line_style(draw::LineStyle::DashDot, 1);
        draw::set_draw_color(enums::Color::Black);
        draw::draw_line(self.get_x(area.xmin) as i32, self.get_y(AXIS_Y) as i32,
            self.get_x(area.xmax) as i32, self.get_y(AXIS_Y) as i32);

        // Axis label
        draw::set_font(enums::Font::Helvetica, 12);
        let yaxis_str = format!("{:.1}", AXIS_Y);
        draw::draw_text2(&yaxis_str,
            self.get_x(area.xmin) as i32 - TICK_SIZE - 2, self.get_y(AXIS_Y) as i32,
            0, 0, enums::Align::Right);

        // Draw plot
        draw::set_line_style(draw::LineStyle::Solid, 1);
        draw::set_draw_color(enums::Color::Red);
        
        for i in 0..y_points.len() {
            draw::draw_xyline(self.get_x(x_points[i]) as i32,
                self.get_y(y_points[i]) as i32,
                self.get_x(x_points[i+1]) as i32);
        }

        self.offs.borrow().end();

        self.redraw();
    }
}

widget_extends!(PlotWidget, widget::Widget, inner);
