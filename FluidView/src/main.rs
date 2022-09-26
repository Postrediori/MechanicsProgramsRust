mod fluid_func;
mod graph_widget;

use fltk::{
    app,
    button,
    input,
    prelude::{DisplayExt, GroupExt, InputExt, WidgetBase, WidgetExt},
    text,
    window
};

fn main() {
    let a = app::App::default();
    app::get_system_colors();

    let mut wind = window::Window::new(100, 100, 700, 500, "Fluid Flow Visual Calculator");

    let mut graph = graph_widget::GraphWidget::new(10, 10, 480, 480);

    let mut inpq = input::FloatInput::new(575, 10, 90, 25, "q = ");
    inpq.set_value("0.5000");

    let buffer = text::TextBuffer::default();
    let mut disp = text::TextDisplay::new(500, 70, 190, 200, "");
    disp.set_buffer(buffer);

    let mut btn_calc = button::Button::new(500, 40, 90, 25, "Calculate");
    btn_calc.set_callback(move |_b| {
        let q_val: f64 = inpq.value().parse::<f64>().expect("Not a number!");

        let mut buffer = disp.buffer().unwrap();
        buffer.set_text("");

        let (lambda1, lambda2) = fluid_func::lambda_q(q_val);

        let lambda1_str = format!("Lambda1: {:.4}\n", lambda1);
        buffer.append(&lambda1_str);

        let lambda2_str = format!("Lambda2: {:.4}\n", lambda2);
        buffer.append(&lambda2_str);

        let eps_str = format!("Epsilon: {:.6}\n", fluid_func::EPS);
        buffer.append(&eps_str);

        graph.set_lines(q_val, lambda1, lambda2);
        graph.redraw();
    });

    wind.end();
    wind.show();

    a.run().unwrap();
}
