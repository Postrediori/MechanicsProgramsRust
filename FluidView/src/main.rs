mod fluid_func;
mod graph_widget;

use fltk::{
    app,
    button,
    input,
    prelude::{DisplayExt, GroupExt, InputExt, WidgetExt},
    text,
    window
};

const WIDTH: i32 = 700;
const HEIGHT: i32 = 500;

fn main() {
    let a = app::App::default();
    app::get_system_colors();

    let mut wind = window::Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label("Fluid Flow Visual Calculator");

    let mut graph = graph_widget::GraphWidget::new(10, 10, HEIGHT-20, HEIGHT-20);

    let mut inpq = input::FloatInput::default()
        .with_size(90, 25).with_pos(WIDTH - 90 - 50, 10)
        .with_label("q = ");
    inpq.set_value("0.5000");

    let mut btn_calc = button::Button::default()
        .with_size(90, 25).below_of(&inpq, 10)
        .with_label("Calculate");

    let buffer = text::TextBuffer::default();
    let mut disp = text::TextDisplay::default()
        .with_size(190, 200)
        .with_pos(graph.x()+graph.w()+10, btn_calc.y()+btn_calc.h()+10);
    disp.set_buffer(buffer);

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