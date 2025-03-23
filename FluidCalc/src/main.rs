#![allow(clippy::cast_sign_loss)]

mod flow_func;
use flow_func::{DirectFunc, InverseFunc};

mod res;
use res::IconsAssets;

use fltk::{
    app, button, frame, group, input, menu, output,
    prelude::{GroupExt, InputExt, MenuExt, WidgetBase, WidgetExt, WindowExt},
    window,
};

const WIDTH: i32 = 400;
const HEIGHT: i32 = 500;

const MARGIN: i32 = 10;
const TABS_HEIGHT: i32 = 30;

#[derive(Clone, Copy)]
enum Message {
    CalculateDirect,
    CalculateInverse,
}

struct DirectFuncTab {
    input: input::FloatInput,
    outputs: Vec<output::Output>,
}

impl DirectFuncTab {
    fn new(
        direct_functions: &Vec<(&str, DirectFunc)>,
        tx: app::Sender<Message>,
        tabs: &group::Tabs,
    ) -> Self {
        let mut group = group::Flex::default()
            .column()
            .with_size(tabs.w(), tabs.h() - TABS_HEIGHT)
            .with_pos(tabs.x(), tabs.y() + TABS_HEIGHT)
            .with_label("vel->func");
        group.set_tooltip("Converting dimensionless velocity lambda into flow functions");
        group.set_margin(5);

        // Input row
        let mut in_lambda;
        {
            let mut row = group::Flex::default_fill().row();

            let label = frame::Frame::default().with_label("lambda = ");
            row.fixed(&label, 75);

            in_lambda = input::FloatInput::default();
            in_lambda.set_value(&format!("{:.4}", 1.0));
            in_lambda.set_tooltip("Dimensionless velocity lambda");

            row.end();

            group.fixed(&row, 30);
        }

        // Calc row
        {
            let mut row = group::Flex::default_fill().row();

            frame::Frame::default();

            let mut btn_calc = button::Button::default()
                .with_size(100, 30)
                .below_of(&in_lambda, 10)
                .with_label("Calculate");
            btn_calc.emit(tx, Message::CalculateDirect);

            row.fixed(&btn_calc, 125);

            frame::Frame::default();

            row.end();

            group.fixed(&row, 30);
        }

        // Rows with function outputs
        let outputs: Vec<output::Output> = direct_functions
            .iter()
            .map(|f| {
                let mut func_row = group::Flex::default_fill().row();

                let label = frame::Frame::default().with_label(&format!("{} =", f.0));
                func_row.fixed(&label, 75);

                let mut output = output::Output::default();
                output.set_value("");
                output.set_tooltip(&format!("Value of flow function {}(lambda)", f.0));

                func_row.end();

                group.fixed(&func_row, 30);

                output
            })
            .collect();

        group.end();

        Self {
            input: in_lambda,
            outputs,
        }
    }
}

/*
 * Inverse functions tab
 */
struct InverseFuncTab {
    func_choice: menu::Choice,
    input: input::FloatInput,
    out_lambda: Vec<output::Output>,
}

impl InverseFuncTab {
    fn new(
        inverse_functions: &Vec<(&str, InverseFunc)>,
        tx: app::Sender<Message>,
        tabs: &group::Tabs,
    ) -> Self {
        let mut group = group::Flex::default()
            .column()
            .with_size(tabs.w(), tabs.h() - TABS_HEIGHT)
            .with_pos(tabs.x(), tabs.y() + TABS_HEIGHT)
            .with_label("func->vel");
        group.set_tooltip("Get dimensionless velocity lambda by value of flow function");
        group.set_margin(5);

        // Input row
        let mut func_choice;
        let mut input;
        {
            let mut row = group::Flex::default_fill().row();

            func_choice = menu::Choice::default().with_size(75, 25);
            for f in inverse_functions {
                func_choice.add_choice(f.0);
            }
            func_choice.set_tooltip("Choose the flow function");
            func_choice.set_value(0);
            row.fixed(&func_choice, 75);

            let label = frame::Frame::default().with_label(" = ");
            row.fixed(&label, 25);

            input = input::FloatInput::default();
            input.set_value(&format!("{:.4}", 1.0));
            input.set_tooltip("Value of the flow function");

            row.end();

            group.fixed(&row, 30);
        }

        // Calc row
        {
            let mut row = group::Flex::default_fill().row();

            frame::Frame::default();

            let mut btn_calc2 = button::Button::default()
                .with_size(100, 30)
                .with_label("Calculate");
            btn_calc2.emit(tx, Message::CalculateInverse);

            row.fixed(&btn_calc2, 125);

            frame::Frame::default();

            row.end();

            group.fixed(&row, 30);
        }

        let outputs = ["lambda1", "lambda2"]
            .iter()
            .map(|s| {
                let mut row = group::Flex::default_fill().row();

                let label = frame::Frame::default().with_label(&format!("{s} ="));
                row.fixed(&label, 75);

                let mut output = output::Output::default();
                output.set_tooltip("Dimensionless velocity lambda");

                row.end();

                group.fixed(&row, 30);

                output
            })
            .collect();

        group.end();

        Self {
            func_choice,
            input,
            out_lambda: outputs,
        }
    }
}

fn main() {
    let direct_functions: Vec<(&str, DirectFunc)> = vec![
        ("tau", flow_func::tau),
        ("pi", flow_func::pi),
        ("eps", flow_func::eps),
        ("q", flow_func::q),
        ("phi", flow_func::phi),
        ("y", flow_func::y),
    ];

    let inverse_functions: Vec<(&str, InverseFunc)> = vec![
        ("tau", flow_func::lambda_tau),
        ("pi", flow_func::lambda_pi),
        ("eps", flow_func::lambda_eps),
        ("q", flow_func::lambda_q),
        ("phi", flow_func::lambda_phi),
        ("y", flow_func::lambda_y),
    ];

    let a = app::App::default();

    let (tx, rx) = app::channel::<Message>();

    let mut wind = window::Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label("Fluid Flow Dynamics Calculator");
    wind.make_resizable(true);

    let tabs = group::Tabs::default()
        .with_size(WIDTH - MARGIN * 2, HEIGHT - MARGIN * 2)
        .center_of_parent();

    // Tab with direct calculations
    let mut direct_tab = DirectFuncTab::new(&direct_functions, tx, &tabs);

    // Tab with inverse calculations
    let mut inverse_tab = InverseFuncTab::new(&inverse_functions, tx, &tabs);

    tabs.end();

    if let Some(img) = IconsAssets::get("FluidCalc32.png") {
        if let Ok(img) = fltk::image::PngImage::from_data(img.data.as_ref()) {
            wind.set_icon(Some(img));
        }
    }

    wind.end();
    wind.show();

    while a.wait() {
        if let Some(msg) = rx.recv() {
            match msg {
                Message::CalculateDirect => {
                    let lambda_val = direct_tab
                        .input
                        .value()
                        .parse::<f64>()
                        .expect("Not a number!");

                    for (i, f) in direct_functions.iter().enumerate() {
                        let result = f.1(lambda_val);
                        direct_tab.outputs[i].set_value(&format!("{result:.4}"));
                    }
                }
                Message::CalculateInverse => {
                    let func_val: f64 = inverse_tab
                        .input
                        .value()
                        .parse::<f64>()
                        .expect("Not a number!");
                    let func_id: i32 = inverse_tab.func_choice.value();

                    let (l1, l2) = inverse_functions[func_id as usize].1(func_val);

                    // Show first solution
                    inverse_tab.out_lambda[0].set_value(&format!("{l1:.4}"));

                    // Show second solution
                    if let Some(lambda) = l2 {
                        inverse_tab.out_lambda[1].activate();
                        inverse_tab.out_lambda[1].set_value(&format!("{lambda:.4}"));
                    } else {
                        inverse_tab.out_lambda[1].set_value("");
                        inverse_tab.out_lambda[1].deactivate();
                    }
                }
            }
        }
    }
}
