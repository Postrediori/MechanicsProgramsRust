mod bessel_func;
mod plot_widget;
use plot_widget::{Area, PlotWidget, PlotFunctionInfo};

use fltk::{*, prelude::*};

const WIDTH: i32 = 700;
const HEIGHT: i32 = 500;

fn main() {
    // Plot parameters
    const DEFAULT_AREA: Area = Area{ xmin: 0.0, xmax: 20.0, ymin: -3.0, ymax: 1.0 };

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

    // App and main window
    let a = app::App::default();
    app::get_system_colors();

    let mut wind = window::Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label("Bessel Function");
    wind.make_resizable(true);

    // Plot widget
    const PLOT_WIDGET_SIZE: i32 = HEIGHT - 20;
    let mut plot = PlotWidget::new(WIDTH - PLOT_WIDGET_SIZE - 10, 10, PLOT_WIDGET_SIZE, PLOT_WIDGET_SIZE);
    plot.set_area(DEFAULT_AREA);

    for p in &plots {
        plot.add_plot(p);
    }


    // Plot bounds
    let mut bounds_frame = group::Group::default()
        .with_size(WIDTH - PLOT_WIDGET_SIZE - 30, 230)
        .with_pos(10, 20)
        .with_label("Plot bounds");
    bounds_frame.set_frame(enums::FrameType::BorderFrame);
    bounds_frame.set_color(enums::Color::Black);

    let mut in_max_x = input::FloatInput::default()
        .with_size(90, 25).with_pos(bounds_frame.x()+65, bounds_frame.y()+10)
        .with_label("x max = ");
    let xmax_str = format!("{:.1}", DEFAULT_AREA.xmax);
    in_max_x.set_value(&xmax_str);

    let mut in_min_x = input::FloatInput::default()
        .with_size(90, 25).below_of(&in_max_x, 10)
        .with_label("x min = ");
    let xmin_str = format!("{:.1}", DEFAULT_AREA.xmin);
    in_min_x.set_value(&xmin_str);

    let mut in_max_y = input::FloatInput::default()
        .with_size(90, 25).below_of(&in_min_x, 10)
        .with_label("y max = ");
    let ymax_str = format!("{:.1}", DEFAULT_AREA.ymax);
    in_max_y.set_value(&ymax_str);

    let mut in_min_y = input::FloatInput::default()
        .with_size(90, 25).below_of(&in_max_y, 10)
        .with_label("y min = ");
    let ymin_str = format!("{:.1}", DEFAULT_AREA.ymin);
    in_min_y.set_value(&ymin_str);

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

    bounds_frame.end();


    // Legend
    let mut legend_frame = group::Group::default()
        .with_size(bounds_frame.w(), 200).below_of(&bounds_frame, 30)
        .with_label("Legend");
    legend_frame.set_frame(enums::FrameType::BorderFrame);
    legend_frame.set_color(enums::Color::Black);

    let mut pack = group::Pack::default()
        .with_size(legend_frame.w()-20, legend_frame.h()-20)
        .with_pos(legend_frame.x()+10, legend_frame.y()+10);
    pack.set_spacing(10);

    for p in &plots {
        let mut legend = frame::Frame::default()
            .with_size(90, 30)
            .with_label(&p.name);
        legend.set_frame(enums::FrameType::FlatBox);
        legend.set_color(enums::Color::lighter(&p.color));
    }

    pack.end();

    legend_frame.end();

    wind.end();
    wind.show();

    a.run().unwrap();
}
