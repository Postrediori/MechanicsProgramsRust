mod bessel_func;
mod plot_widget;

use fltk::{*, prelude::*};

fn main() {
    let a = app::App::default();
    app::get_system_colors();

    let mut wind = window::Window::new(100, 100, 700, 500, "Bessel Function");

    let mut in_max_x = input::FloatInput::new(60, 30, 90, 25, "x max = ");
    in_max_x.set_value("20.0");

    let mut in_min_x = input::FloatInput::new(60, 60, 90, 25, "x min = ");
    in_min_x.set_value("0.0");

    let mut in_max_y = input::FloatInput::new(60, 90, 90, 25, "y max = ");
    in_max_y.set_value("1.0");

    let mut in_min_y = input::FloatInput::new(60, 120, 90, 25, "y min = ");
    in_min_y.set_value("-3.0");

    let mut plot = plot_widget::PlotWidget::new(210, 10, 480, 480);

    let mut btn_redraw = button::Button::new(60, 150, 90, 25, "Redraw");
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
