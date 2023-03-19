mod bessel_func;
mod plot_widget;
use plot_widget::{Area, PlotWidget, PlotFunctionInfo};

use fltk::{*, prelude::*};

const WIDTH: i32 = 700;
const HEIGHT: i32 = 500;

fn main() {
    let plots: Vec<PlotFunctionInfo> = [
        PlotFunctionInfo{
            f: bessel_func::y0_1,
            color: enums::Color::from_u32(0xa00000),
            name: "Integration".to_string()
        },
        PlotFunctionInfo{
            f: bessel_func::y0_2,
            color: enums::Color::from_u32(0x99ccff),
            name: "Infinite series".to_string()
        },
    ].to_vec();

    let default_area = Area{ xmin: 0.0, xmax: 20.0, ymin: -3.0, ymax: 1.0 };

    let a = app::App::default();
    app::get_system_colors();

    let mut wind = window::Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label("Bessel Function");

    let mut bounds_frame = frame::Frame::default()
        .with_size(190, 230).with_pos(10, 10);
    bounds_frame.set_frame(enums::FrameType::BorderFrame);
    bounds_frame.set_color(enums::Color::Dark3);

    let bounds_frame_title = frame::Frame::default()
        .with_size(90, 25).with_pos(bounds_frame.x()+65, bounds_frame.y()+10)
        .with_label("Plot bounds");

    let mut in_max_x = input::FloatInput::default()
        .with_size(90, 25).below_of(&bounds_frame_title, 10)
        .with_label("x max = ");
    let xmax_str = format!("{:.1}", default_area.xmax);
    in_max_x.set_value(&xmax_str);

    let mut in_min_x = input::FloatInput::default()
        .with_size(90, 25).below_of(&in_max_x, 10)
        .with_label("x min = ");
    let xmin_str = format!("{:.1}", default_area.xmin);
    in_min_x.set_value(&xmin_str);

    let mut in_max_y = input::FloatInput::default()
        .with_size(90, 25).below_of(&in_min_x, 10)
        .with_label("y max = ");
    let ymax_str = format!("{:.1}", default_area.ymax);
    in_max_y.set_value(&ymax_str);

    let mut in_min_y = input::FloatInput::default()
        .with_size(90, 25).below_of(&in_max_y, 10)
        .with_label("y min = ");
    let ymin_str = format!("{:.1}", default_area.ymin);
    in_min_y.set_value(&ymin_str);

    let mut plot = PlotWidget::new(210, 10, HEIGHT - 20, HEIGHT - 20);
    plot.set_area(default_area);

    for p in &plots {
        plot.add_plot(p);
    }

    let mut btn_redraw = button::Button::default()
        .with_size(90, 25).below_of(&in_min_y, 10)
        .with_label("Update");

    btn_redraw.set_callback(move |_b| {
        let max_x: f64 = in_max_x.value().parse::<f64>().expect("Not a number!");
        let min_x: f64 = in_min_x.value().parse::<f64>().expect("Not a number!");
        let max_y: f64 = in_max_y.value().parse::<f64>().expect("Not a number!");
        let min_y: f64 = in_min_y.value().parse::<f64>().expect("Not a number!");

        plot.set_area(plot_widget::Area{xmin: min_x, xmax: max_x, ymin: min_y, ymax: max_y});
        plot.redraw();
    });

    wind.end();
    wind.show();

    a.run().unwrap();
}
