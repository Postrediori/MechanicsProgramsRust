mod bessel_func;
mod plot_widget;

use fltk::{*, prelude::*};

const WIDTH: i32 = 700;
const HEIGHT: i32 = 500;

fn main() {
    let a = app::App::default();
    app::get_system_colors();

    let mut wind = window::Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label("Bessel Function");

    let mut in_max_x = input::FloatInput::default()
        .with_size(90, 25).with_pos(60, 30)
        .with_label("x max = ");
    in_max_x.set_value("20.0");

    let mut in_min_x = input::FloatInput::default()
        .with_size(90, 25).below_of(&in_max_x, 10)
        .with_label("x min = ");
    in_min_x.set_value("0.0");

    let mut in_max_y = input::FloatInput::default()
        .with_size(90, 25).below_of(&in_min_x, 10)
        .with_label("y max = ");
    in_max_y.set_value("1.0");

    let mut in_min_y = input::FloatInput::default()
        .with_size(90, 25).below_of(&in_max_y, 10)
        .with_label("y min = ");
    in_min_y.set_value("-3.0");

    let mut plot = plot_widget::PlotWidget::new(210, 10, HEIGHT - 20, HEIGHT - 20);

    let mut btn_redraw = button::Button::default()
        .with_size(90, 25).below_of(&in_min_y, 10)
        .with_label("Redraw");
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
