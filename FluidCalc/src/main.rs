mod flow_func;

use fltk::{
    app,
    button,
    input,
    group,
    menu,
    output,
    prelude::{InputExt, GroupExt, MenuExt, WidgetBase, WidgetExt},
    window,
};

fn main() {
    let a = app::App::default();
    
    let mut wind = window::Window::new(100, 100, 400, 500, "Fluid Flow Dynamics Calculator");

    let tabs = group::Tabs::new(10, 10, 380, 480, "");

    // Group 1: vel -> func calculations

    let group1 = group::Group::new(10, 40, 380, 480, "vel->func");

    let mut in_lambda = input::FloatInput::new(150, 70, 150, 25, "lambda = ");
    let _ = in_lambda.set_value("1.0000");

    let mut btn_calc = button::Button::new(150, 105, 100, 30, "Calculate");

    let mut out_tau = output::Output::new(150, 145, 150, 25, "tau = ");
    let mut out_pi = output::Output::new(150, 175, 150, 25, "pi = ");
    let mut out_eps = output::Output::new(150, 205, 150, 25, "eps = ");
    let mut out_q = output::Output::new(150, 235, 150, 25, "q = ");
    
    let mut out_phi = output::Output::new(150, 270, 150, 25, "phi = ");
    let mut out_y = output::Output::new(150, 300, 150, 25, "y = ");

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

    let group2 =  group::Group::new(10, 40, 380, 480, "func->vel");

    let mut func_choice = menu::Choice::new(50, 70, 75, 25, "");
    func_choice.add_choice("tau");
    func_choice.add_choice("pi");
    func_choice.add_choice("eps");
    func_choice.add_choice("q");
    func_choice.add_choice("phi");
    func_choice.add_choice("y");
    func_choice.set_value(0);

    let mut in_func = input::FloatInput::new(150, 70, 150, 25, " = ");
    in_func.set_value("1.0000");

    let mut btn_calc2 = button::Button::new(150, 105, 100, 30, "Calculate");

    let mut out_lambda1 = output::Output::new(150, 145, 150, 25, "lambda1 = ");
    let mut out_lambda2 = output::Output::new(150, 175, 150, 25, "lambda2 = ");

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
