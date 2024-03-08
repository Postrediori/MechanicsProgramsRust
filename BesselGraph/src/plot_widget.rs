use fltk::{
    draw,
    enums,
    prelude::{WidgetBase, WidgetExt},
    widget,
    widget_extends
};

use std::cell::RefCell;
use std::rc::Rc;

pub struct Point {
    x: f64,
    y: f64,
}

pub type Line = (Point, Point);

// pub type PlotPoints = Vec<Point>;
pub type PlotLines = Vec<Line>;

#[derive(Copy, Clone)]
pub struct Area {
    pub xmin: f64,
    pub ymin: f64,
    pub xmax: f64,
    pub ymax: f64,
}

impl Area {
    fn is_inside(&self, p: &Point) -> bool {
        p.x>self.xmin && p.x<self.xmax && p.y>self.ymin && p.y<self.ymax
    }
}

type PlotFunction = fn(f64) -> f64;


#[derive(Clone)]
pub struct PlotFunctionInfo {
    pub f: PlotFunction,
    pub color: enums::Color,
    pub name: String
}

impl PlotFunctionInfo {
    pub fn calc_points(&self, line_count: i32, area: &Area) -> PlotLines {
        let px: f64 = (area.xmax - area.xmin) / (line_count as f64);
        (0..line_count).map(|i| {
            let x1: f64 = area.xmin + px * (i as f64);
            let x2: f64 = area.xmin + px * ((i + 1) as f64);
            (Point{ x: x1, y: (self.f)(x1) }, Point{ x: x2, y: (self.f)(x2) })
        }).collect::<PlotLines>()
    }
}

pub struct PlotWidget {
    inner: widget::Widget,
    area: Rc<RefCell<Area>>,
    plots: Rc<RefCell<Vec<(PlotFunctionInfo, PlotLines)>>>
}

impl PlotWidget {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        let mut inner = widget::Widget::default().with_pos(x, y).with_size(width, height);

        let area = Area{ xmin: -1.0, xmax: 1.0, ymin: -1.0, ymax: 1.0 };
        let area = Rc::from(RefCell::from(area));

        let plots: Vec<(PlotFunctionInfo, PlotLines)> = Vec::new();
        let plots = Rc::from(RefCell::from(plots));

        const MARGIN: i32 = 20;
        const TICK_SIZE: i32 = 10;

        let tick_count: i32 = 20;

        const BOUNDS_COLOR: enums::Color = enums::Color::Dark3;
        const BG_COLOR: enums::Color = enums::Color::White;
        const PLOT_BOUNDS_COLOR: enums::Color = enums::Color::Black;
        const TICKS_COLOR: enums::Color = enums::Color::Black;
        const AXES_COLOR: enums::Color = enums::Color::Black;

        let area_val = area.clone();
        let plots_val = plots.clone();

        inner.draw(move |i| {
            let area = area_val.borrow();
            let plots = plots_val.borrow();

            // Calculate pixel scale
            let scale_x: f64 = (area.xmax - area.xmin) / ((i.w() - MARGIN * 3 - TICK_SIZE) as f64);
            let scale_y: f64 = (area.ymax - area.ymin) / ((i.h() - MARGIN * 2 - TICK_SIZE) as f64);

            // Helper functions
            let get_x = |x: f64| -> i32 { ((x - area.xmin) / scale_x) as i32 + i.x() + MARGIN * 2 + TICK_SIZE };
            let get_y = |y: f64| -> i32 { ((area.ymax - y) / scale_y) as i32 + i.y() + MARGIN };
            let get_coord = |x: f64, y:f64| -> draw::Coord<i32> { draw::Coord::<i32>(get_x(x), get_y(y)) };

            // Calculate ticks

            // X-Axis ticks
            let mut ticks_x: Vec<Line> = Vec::new();
            ticks_x.reserve((tick_count + 1) as usize);
            let tx: f64 = (area.xmax - area.xmin) / (tick_count as f64);
            for i in 0..tick_count+1 {
                let xc: f64 = area.xmin + tx * (i as f64);
                let tick_scale: f64 = if i % 2 == 0 { 1.0 } else { 0.5 };
                ticks_x.push( (
                    Point{ x: xc, y: area.ymin },
                    Point{ x: xc, y: area.ymin - (TICK_SIZE as f64) * scale_y * tick_scale }
                ) );
            }

            // Y-Axis ticks
            let mut ticks_y: Vec<Line> = Vec::new();
            ticks_y.reserve((tick_count + 1) as usize);
            let ty: f64 = (area.ymax - area.ymin) / (tick_count as f64);
            for i in 0..tick_count+1 {
                let yc: f64 = area.ymin + ty * (i as f64);
                let tick_scale: f64 = if i % 2 == 0 { 1.0 } else { 0.5 };
                ticks_y.push( (
                    Point{ x: area.xmin, y: yc },
                    Point{ x: area.xmin - (TICK_SIZE as f64) * scale_x * tick_scale , y: yc }
                ) );
            }

            // Set default style for ranges and ticks
            draw::set_line_style(draw::LineStyle::Solid, 1);

            // Clean draw area with backgorund color
            draw::draw_rect_fill(i.x(), i.y(), i.w(), i.h(), BG_COLOR);

            // Widget bounds
            draw::set_draw_color(BOUNDS_COLOR);
            draw::draw_rect(i.x(), i.y(), i.w(), i.h());
            
            // Draw bounding box
            draw::set_draw_color(PLOT_BOUNDS_COLOR);
            draw::draw_loop3(
                get_coord(area.xmin, area.ymin),
                get_coord(area.xmax, area.ymin),
                get_coord(area.xmax, area.ymax),
                get_coord(area.xmin, area.ymax)
            );

            // Draw origin axes (if visible)
            const AXIS_X: f64 = 0.0;
            const AXIS_Y: f64 = 0.0;
            draw::set_draw_color(AXES_COLOR);
            draw::set_line_style(draw::LineStyle::DashDotDot, 1);
            
            if area.xmin<AXIS_X && AXIS_X<area.xmax {
                draw::draw_line(
                    get_x(AXIS_X), get_y(area.ymin),
                    get_x(AXIS_X), get_y(area.ymax));
            }

            if area.ymin<AXIS_Y && AXIS_Y<area.ymax {
                draw::draw_line(
                    get_x(area.xmin), get_y(AXIS_Y),
                    get_x(area.xmax), get_y(AXIS_Y));
            }

            // Draw ticks
            draw::set_draw_color(TICKS_COLOR);
            draw::set_line_style(draw::LineStyle::Solid, 1);

            for t in &ticks_x {
                draw::draw_line(
                    get_x(t.0.x), get_y(t.0.y),
                    get_x(t.1.x), get_y(t.1.y));
            }

            for t in &ticks_y {
                draw::draw_line(
                    get_x(t.0.x), get_y(t.0.y),
                    get_x(t.1.x), get_y(t.1.y));
            }

            // Draw ranges
            draw::set_draw_color(PLOT_BOUNDS_COLOR);
            draw::set_font(enums::Font::Helvetica, 12);

            draw::draw_text2(format!("{:.2}", area.xmin).as_str(),
                get_x(area.xmin),
                get_y(area.ymin - scale_y * (TICK_SIZE as f64 * 1.25)),
                0, 0, enums::Align::TopLeft);

            draw::draw_text2(format!("{:.2}", area.xmax).as_str(),
                get_x(area.xmax),
                get_y(area.ymin - scale_y * (TICK_SIZE as f64 * 1.25)),
                0, 0, enums::Align::TopRight);

            draw::draw_text2(format!("{:.2}", area.ymin).as_str(),
                get_x(area.xmin - scale_x * (TICK_SIZE as f64 * 1.25)),
                get_y(area.ymin),
                0, 0, enums::Align::BottomRight);

            draw::draw_text2(format!("{:.2}", area.ymax).as_str(),
                get_x(area.xmin - scale_x * (TICK_SIZE as f64 * 1.25)),
                get_y(area.ymax),
                0, 0, enums::Align::TopRight);

            // Draw plots
            if !plots.is_empty() {
                let mut width: i32 = (plots.len() * 3 - 1) as i32; // Width of the lower-most plot
                for p in plots.iter() {
                    draw::set_draw_color(p.0.color);
                    draw::set_line_style(draw::LineStyle::Solid | draw::LineStyle::CapRound, width);

                    for l in &p.1 {
                        if area.is_inside(&l.0) && area.is_inside(&l.1) {
                            draw::draw_line(get_x(l.0.x), get_y(l.0.y), get_x(l.1.x), get_y(l.1.y));
                        }
                    }

                    width -= 3; // Decrease width of plots lines as it goes to the top
                }
            }
            
            draw::set_line_style(draw::LineStyle::Solid, 0);
        });

        inner.handle(move |_i, ev| match ev {
            _ => false,
        });

        Self {
            inner,
            area,
            plots
        }
    }

    pub fn set_area(&mut self, new_area: Area) {
        *self.area.borrow_mut() = new_area;
    }

    pub fn get_area(&self) -> Area {
        *self.area.borrow()
    }

    pub fn clear_plots(&mut self) {
        self.plots.borrow_mut().clear();
    }

    pub fn add_plot(&mut self, new_plot: (PlotFunctionInfo, PlotLines)) {
        self.plots.borrow_mut().push((new_plot.0.clone(), new_plot.1));
    }

    pub fn redraw(&mut self) {
        self.inner.redraw();
    }

}

widget_extends!(PlotWidget, widget::Widget, inner);
