use fltk::{
    draw,
    enums,
    prelude::{WidgetBase, WidgetExt},
    widget,
    widget_extends
};

use std::cell::RefCell;
use std::rc::Rc;

use crate::fluid_func;

struct Point {
    x: f64,
    y: f64,
}

struct Tick {
    begin: Point,
    end: Point,
}

struct Area {
    xmin: f64,
    ymin: f64,
    xmax: f64,
    ymax: f64,
}

pub struct GraphWidget {
    inner: widget::Widget,
    q: Rc<RefCell<f64>>,
    lambda1: Rc<RefCell<f64>>,
    lambda2: Rc<RefCell<f64>>
}

impl GraphWidget {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        let mut inner = widget::Widget::default().with_pos(x, y).with_size(width, height);

        let max_lambda: f64 = ((fluid_func::K + 1.0) / (fluid_func::K - 1.0)).sqrt();

        let q: f64 = 0.0;
        let lambda1: f64 = 0.0;
        let lambda2: f64 = 0.0;
        let area: Area = Area{ xmin: 0.0, ymin: 0.0, xmax: max_lambda, ymax: 1.0};

        let q = Rc::from(RefCell::from(q));
        let lambda1 = Rc::from(RefCell::from(lambda1));
        let lambda2 = Rc::from(RefCell::from(lambda2));

        let q_val = q.clone();
        let lambda1_val = lambda1.clone();
        let lambda2_val = lambda2.clone();

        const MARGIN: i32 = 20;
        const TICK_SIZE: i32 = 10;

        let point_count: i32 = 500;
        let tick_count: i32 = 20;

        let bg_color = enums::Color::White;
        let bounds_color = enums::Color::Black;
        let ticks_color = enums::Color::Black;

        let plot_color = enums::Color::from_rgb(255, 25, 50);
        let q_line_color = enums::Color::from_rgb(32, 128, 32);
        let lambda_line_color = enums::Color::from_rgb(0, 64, 192);


        // Calculate plot points
        let mut point_vec: Vec<Point> = Vec::new();
        let px: f64 = (area.xmax - area.xmin) / (point_count as f64);
        for i in 0..point_count+1 {
            let xc: f64 = area.xmin + px * (i as f64);
            point_vec.push(Point{ x: xc, y: fluid_func::q(xc) });
        }

        inner.draw(move |i| {
            // Calculate pixel scale
            let scale_x: f64 = (area.xmax - area.xmin) / ((i.w() - MARGIN * 2 - TICK_SIZE) as f64);
            let scale_y: f64 = (area.ymax - area.ymin) / ((i.h() - MARGIN * 2 - TICK_SIZE) as f64);

            // Helper functions
            let get_x = |x: f64| -> i32 { ((x - area.xmin) / scale_x) as i32 + i.x() + MARGIN + TICK_SIZE };
            let get_y = |y: f64| -> i32 { ((area.ymax - y) / scale_y) as i32 + i.y() + MARGIN };
            let get_coord = |x: f64, y:f64| -> draw::Coord<i32> { draw::Coord::<i32>(get_x(x), get_y(y)) };

            // Calculate ticks

            // X-Axis ticks
            let mut ticks_x: Vec<Tick> = Vec::new();
            let tx: f64 = (area.xmax - area.xmin) / (tick_count as f64);
            for i in 0..tick_count+1 {
                let xc: f64 = area.xmin + tx * (i as f64);
                let tick_scale: f64 = if i % 2 == 0 { 1.0 } else { 0.5 };
                ticks_x.push(Tick {
                    begin: Point{ x: xc, y: area.ymin },
                    end: Point{ x: xc, y: area.ymin - (TICK_SIZE as f64) * scale_y * tick_scale }
                } );
            }

            // Y-Axis ticks
            let mut ticks_y: Vec<Tick> = Vec::new();
            let ty: f64 = (area.ymax - area.ymin) / (tick_count as f64);
            for i in 0..tick_count+1 {
                let yc: f64 = area.ymin + ty * (i as f64);
                let tick_scale: f64 = if i % 2 == 0 { 1.0 } else { 0.5 };
                ticks_y.push(Tick {
                    begin: Point{ x: area.xmin, y: yc },
                    end: Point{ x: area.xmin - (TICK_SIZE as f64) * scale_x * tick_scale , y: yc }
                } );
            }

            // Clean draw area with backgorund color
            draw::draw_rect_fill(i.x(), i.y(), i.w(), i.h(), bg_color);

            // Draw bounding box
            draw::set_draw_color(bounds_color);
            draw::draw_loop3(
                get_coord(area.xmin, area.ymin),
                get_coord(area.xmax, area.ymin),
                get_coord(area.xmax, area.ymax),
                get_coord(area.xmin, area.ymax)
            );

            // Draw ticks
            draw::set_draw_color(ticks_color);

            for t in &ticks_x {
                draw::draw_line(
                    get_x(t.begin.x), get_y(t.begin.y),
                    get_x(t.end.x), get_y(t.end.y));
            }

            for t in &ticks_y {
                draw::draw_line(
                    get_x(t.begin.x), get_y(t.begin.y),
                    get_x(t.end.x), get_y(t.end.y));
            }

            // Draw ranges
            draw::set_draw_color(bounds_color);
            draw::set_font(enums::Font::Helvetica, 12);

            let xmin_str = format!("{:.1}", area.xmin);
            draw::draw_text2(&xmin_str,
                get_x(area.xmin),
                get_y(area.ymin - scale_y * ((MARGIN + TICK_SIZE) as f64)),
                0, 0, enums::Align::BottomLeft);

            let xmax_str = format!("{:.1}", area.xmax);
            draw::draw_text2(&xmax_str,
                get_x(area.xmax),
                get_y(area.ymin - scale_y * ((MARGIN + TICK_SIZE) as f64)),
                0, 0, enums::Align::BottomRight);

            let ymin_str = format!("{:.1}", area.ymin);
            draw::draw_text2(&ymin_str,
                get_x(area.xmin - scale_x * (MARGIN + TICK_SIZE) as f64),
                get_y(area.ymin),
                0, 0, enums::Align::BottomLeft);

            let ymax_str = format!("{:.1}", area.ymax);
            draw::draw_text2(&ymax_str,
                get_x(area.xmin - scale_x * (MARGIN + TICK_SIZE) as f64),
                get_y(area.ymax),
                0, 0, enums::Align::TopLeft);

            // Draw labels
            let x_label = "lambda";
            let y_label = "q";

            draw::set_font(enums::Font::HelveticaBold, 14);

            draw::draw_text2(&x_label,
                get_x(area.xmin + ((area.xmax - area.xmin) * 0.5) as f64),
                get_y(area.ymin - scale_y * ((TICK_SIZE as f64) * 1.5)),
            0, 0, enums::Align::Center | enums::Align::Top);

            draw::draw_text2(&y_label,
                get_x(area.xmin - scale_x * (TICK_SIZE as f64) * 1.5),
                get_y(area.ymin + ((area.ymax - area.ymin) * 0.5) as f64),
            0, 0, enums::Align::Right);

            // Draw plot
            draw::set_draw_color(plot_color);
            draw::begin_line();
            for c in &point_vec {
                draw::vertex(get_x(c.x) as f64, get_y(c.y) as f64);
            }
            draw::end_line();

            // Draw lines
            let q: f64 = *q_val.borrow();
            if q > 0.0 {
                draw::set_draw_color(q_line_color);
                draw::draw_line(get_x(area.xmin), get_y(q),
                    get_x(area.xmax), get_y(q));

                let lambda1: f64 = *lambda1_val.borrow();
                let lambda2: f64 = *lambda2_val.borrow();

                draw::set_draw_color(lambda_line_color);
                draw::draw_line(
                    get_x(lambda1), get_y(area.ymin),
                    get_x(lambda1), get_y(q));
                draw::draw_line(
                    get_x(lambda2), get_y(area.ymin),
                    get_x(lambda2), get_y(q));
            }
        });

        inner.handle(move |_i, ev| match ev {
            _ => false,
        });

        Self {
            inner,
            q,
            lambda1,
            lambda2,
        }
    }

    pub fn set_lines(&mut self, new_q: f64, new_lambda1: f64, new_lambda2: f64) {
        *self.q.borrow_mut() = new_q;
        *self.lambda1.borrow_mut() = new_lambda1;
        *self.lambda2.borrow_mut() = new_lambda2;
    }
}

widget_extends!(GraphWidget, widget::Widget, inner);
