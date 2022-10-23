use fltk::{
    draw,
    enums,
    prelude::{WidgetBase, WidgetExt},
    widget,
    widget_extends
};

use crate::bessel_func;

use std::cell::Ref;
use std::cell::RefCell;
use std::rc::Rc;

struct Point {
    x: f64,
    y: f64,
}

struct Tick {
    begin: Point,
    end: Point,
}

#[derive(Copy, Clone)]
pub struct Area {
    pub xmin: f64,
    pub ymin: f64,
    pub xmax: f64,
    pub ymax: f64,
}

pub struct PlotWidget {
    inner: widget::Widget,
    area: Rc<RefCell<Area>>
}

fn calc_points(points: &mut Vec<Point>, point_count: i32, area: Ref<Area>, f: fn(f64) -> f64) {
    let px: f64 = (area.xmax - area.xmin) / (point_count as f64);
    for i in 0..point_count+1 {
        let xc: f64 = area.xmin + px * (i as f64);
        points.push(Point{ x: xc, y: f(xc) });
    }
}

impl PlotWidget {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        let mut inner = widget::Widget::default().with_pos(x, y).with_size(width, height);

        let area = Rc::from(RefCell::from(Area{ xmin: 0.0, xmax: 20.0, ymin: -3.0, ymax: 1.0}));
        let area_val = area.clone();

        const MARGIN: i32 = 20;
        const TICK_SIZE: i32 = 10;

        let point_count: i32 = 500;
        let tick_count: i32 = 20;

        let bg_color = enums::Color::White;
        let bounds_color = enums::Color::Black;
        let ticks_color = enums::Color::Black;

        let plot_color_1 = enums::Color::from_rgb(0xa0, 0, 0);
        let plot_color_2 = enums::Color::from_rgb(0x99, 0xcc, 0xff);
        
        let plot_width_1 = 5;
        let plot_width_2 = 2;

        inner.draw(move |i| {
            let area = area_val.borrow();

            // Calculate plot points
            let mut point_vec_1: Vec<Point> = Vec::new();
            calc_points(&mut point_vec_1, point_count, area_val.borrow(), bessel_func::y0_1);

            let mut point_vec_2: Vec<Point> = Vec::new();
            calc_points(&mut point_vec_2, point_count, area_val.borrow(), bessel_func::y0_2);

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

            // Set default style for ranges and ticks
            draw::set_line_style(draw::LineStyle::Solid, 1);

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
            draw::draw_text(&xmin_str,
                get_x(area.xmin),
                get_y(area.ymin - scale_y * ((MARGIN + TICK_SIZE) as f64)));

            let xmax_str = format!("{:.1}", area.xmax);
            draw::draw_text(&xmax_str,
                get_x(area.xmax),
                get_y(area.ymin - scale_y * ((MARGIN + TICK_SIZE) as f64)));

            let ymin_str = format!("{:.1}", area.ymin);
            draw::draw_text(&ymin_str,
                get_x(area.xmin - scale_x * (MARGIN + TICK_SIZE) as f64),
                get_y(area.ymin));

            let ymax_str = format!("{:.1}", area.ymax);
            draw::draw_text(&ymax_str,
                get_x(area.xmin - scale_x * (MARGIN + TICK_SIZE) as f64),
                get_y(area.ymax));

            // Draw plots
            draw::set_draw_color(plot_color_1);
            draw::set_line_style(draw::LineStyle::Solid, plot_width_1);
            draw::begin_line();
            for c in &point_vec_1 {
                if c.x>area.xmin && c.x<area.xmax && c.y>area.ymin && c.y<area.ymax {
                    draw::vertex(get_x(c.x) as f64, get_y(c.y) as f64);
                }
            }
            draw::end_line();

            draw::set_draw_color(plot_color_2);
            draw::set_line_style(draw::LineStyle::Solid, plot_width_2);
            draw::begin_line();
            for c in &point_vec_2 {
                if c.x>area.xmin && c.x<area.xmax && c.y>area.ymin && c.y<area.ymax {
                    draw::vertex(get_x(c.x) as f64, get_y(c.y) as f64);
                }
            }
            draw::end_line();
        });

        inner.handle(move |_i, ev| match ev {
            _ => false,
        });

        Self {
            inner,
            area
        }
    }

    pub fn set_area(&mut self, new_area: Area) {
        *self.area.borrow_mut() = new_area;
    }

}

widget_extends!(PlotWidget, widget::Widget, inner);
