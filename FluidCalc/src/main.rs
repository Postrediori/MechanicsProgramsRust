mod flow_func;

use fltk::{
    app,
    button,
    input,
    group,
    menu,
    output,
    prelude::{InputExt, GroupExt, MenuExt, WidgetExt},
    window,
};

const WIDTH: i32 = 400;
const HEIGHT: i32 = 500;

fn main() {
    let a = app::App::default();
    
    let mut wind = window::Window::default()
        .with_size(WIDTH, HEIGHT).with_label("Fluid Flow Dynamics Calculator");
    wind.make_resizable(true);

    let tabs = group::Tabs::default()
        .with_size(WIDTH - 20, HEIGHT - 20).center_of_parent();

    // Group 1: vel -> func calculations

    let group1 = group::Group::default()
        .with_size(tabs.w(), tabs.h() - 30).with_pos(tabs.x(), tabs.y() + 30)
        .with_label("vel->func");

    let mut in_lambda = input::FloatInput::default()
        .with_size(150, 25).with_pos(group1.x() + 140, group1.y() + 30)
        .with_label("lambda = ");
    let _ = in_lambda.set_value("1.0000");

    let mut btn_calc = button::Button::default()
        .with_size(100, 30).below_of(&in_lambda, 10)
        .with_label("Calculate");

    let mut out_tau = output::Output::default()
        .with_size(150, 25).below_of(&btn_calc, 10)
        .with_label("tau = ");
    let mut out_pi = output::Output::default()
        .with_size(150, 25).below_of(&out_tau, 10)
        .with_label("pi = ");
    let mut out_eps = output::Output::default()
        .with_size(150, 25).below_of(&out_pi, 10)
        .with_label("eps = ");
    let mut out_q = output::Output::default()
        .with_size(150, 25).below_of(&out_eps, 10)
        .with_label("q = ");
    
    let mut out_phi = output::Output::default()
        .with_size(150, 25).below_of(&out_q, 10)
        .with_label("phi = ");
    let mut out_y = output::Output::default()
        .with_size(150, 25).below_of(&out_phi, 10)
        .with_label("y = ");

    btn_calc.set_callback(move |_b| {
        let lambda_val = in_lambda.value().parse::<f64>().expect("Not a number!");
        
        let tau_str = format!("{:.4}", flow_func::tau(lambda_val));
        out_tau.set_value(&tau_str);
        
        let pi_str = format!("{:.4}", flow_func::pi(lambda_val));
        out_pi.set_value(&pi_str);
        
        let eps_str = format!("{:.4}", flow_func::eps(lambda_val));
        out_eps.set_value(&eps_str);
        
        let q_str = format!("{:.4}", flow_func::q(lambda_val));
        out_q.set_value(&q_str);
        
        let phi_str = format!("{:.4}", flow_func::phi(lambda_val));
        out_phi.set_value(&phi_str);
        
        let y_str = format!("{:.4}", flow_func::y(lambda_val));
        out_y.set_value(&y_str);
    });

    group1.end();

    // Group 2: func -> vel calculations

    let group2 =  group::Group::default()
        .with_size(tabs.w(), tabs.h() - 30).with_pos(tabs.x(), tabs.y() + 30)
        .with_label("func->vel");

    let mut in_func = input::FloatInput::default()
        .with_size(150, 25).with_pos(group2.x() + 140, group2.y() + 30)
        .with_label(" = ");
    in_func.set_value("1.0000");

    let mut func_choice = menu::Choice::default()
        .with_size(75, 25).left_of(&in_func, 30);
    func_choice.add_choice("tau");
    func_choice.add_choice("pi");
    func_choice.add_choice("eps");
    func_choice.add_choice("q");
    func_choice.add_choice("phi");
    func_choice.add_choice("y");
    func_choice.set_value(0);

    let mut btn_calc2 = button::Button::default()
        .with_size(100, 30).below_of(&in_func, 10)
        .with_label("Calculate");

    let mut out_lambda1 = output::Output::default()
        .with_size(150, 25).below_of(&btn_calc2, 10)
        .with_label("lambda1 = ");
    let mut out_lambda2 = output::Output::default()
        .with_size(150, 25).below_of(&out_lambda1, 10)
        .with_label("lambda2 = ");

    btn_calc2.set_callback(move |_b| {
        let func_val: f64 = in_func.value().parse::<f64>().expect("Not a number!");
        let func_id: i32 = func_choice.value();

        let l1: f64;
        let mut l2: f64 = -1.0;

        match func_id {
            0 => {
                l1 = flow_func::lambda_tau(func_val)
            },
            1 => {
                l1 = flow_func::lambda_pi(func_val)
            },
            2 => {
                l1 = flow_func::lambda_eps(func_val)
            },
            3 => {
                (l1, l2) = flow_func::lambda_q(func_val)
            },
            4 => {
                (l1, l2) = flow_func::lambda_phi(func_val)
            },
            5 => {
                l1 = flow_func::lambda_y(func_val)
            },
            _ => unreachable!()
        }

        let lambda1_str = format!("{:.4}", l1);
        out_lambda1.set_value(&lambda1_str);
        
        if l2 > 0.0 {
            let lambda2_str = format!("{:.4}", l2);
            out_lambda2.set_value(&lambda2_str);
        }
        else {
            out_lambda2.set_value("");
        }
    });

    group2.end();

    //

    tabs.end();
    
    wind.end();
    wind.show();

    a.run().unwrap();
} 
