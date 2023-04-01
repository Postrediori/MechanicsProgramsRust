use fltk::{*, prelude::*};

use crate::{plot_widget::PlotWidget, pipe_model::{PipeModel, BOUNDARY_OPEN, BOUNDARY_SEALED}};

const MARGIN: i32 = 10;
pub struct MainWindow {
    wind: window::Window,
    uw_plot: PlotWidget,
    pw_plot: PlotWidget,
    choice_ux: menu::Choice,
    choice_px: menu::Choice,
    choice_left: menu::Choice,
    choice_right: menu::Choice,
    in_len: input::FloatInput,
    in_n: input::IntInput,
    in_a: input::FloatInput,
    in_rho: input::FloatInput,
    in_sigma: input::FloatInput,
    pub btn_apply: button::Button,
    pub btn_step: button::Button,
    pub btn_start_stop: button::Button,
}

impl MainWindow {
    pub fn make_window(w: i32, h: i32, title: &str) -> Self {
        let wind = window::Window::default()
            .with_size(w, h)
            .with_label(title);

        let plot_widget_w: i32 = h - MARGIN * 2;
        let plot_widget_h: i32 = (plot_widget_w - MARGIN) / 2;

        let uw_plot = PlotWidget::new(MARGIN, MARGIN,
            plot_widget_w, plot_widget_h, "U(x)");

        let pw_plot = PlotWidget::new(MARGIN, uw_plot.y() + uw_plot.h() + MARGIN,
            plot_widget_w, plot_widget_h, "P(x)");

        let mut g_settings = group::Group::default()
            .with_size(195, 320)
            .with_pos(MARGIN * 2 + plot_widget_w, MARGIN * 2)
            .with_label("Settings").with_align(enums::Align::Top);
        g_settings.set_frame(enums::FrameType::ShadowBox);
        
        let mut choice_ux = menu::Choice::default()
            .with_size(100, 25)
            .with_pos(g_settings.x() + 75, g_settings.y() + MARGIN)
            .with_label("U(X)");
        choice_ux.set_tooltip("Initial condition for velocities in the pipe");
        choice_ux.set_text_color(enums::Color::DarkBlue);
        choice_ux.set_selection_color(enums::Color::DarkBlue);
        choice_ux.add_choice("▄▄▄▄▄▄");
        choice_ux.add_choice("██▄▄██");
        choice_ux.add_choice("██▄▄▄▄");
        choice_ux.add_choice("▄▄██▄▄");
        choice_ux.add_choice("▄▄▄▄██");
        choice_ux.add_choice("▄▄▄███");
        choice_ux.add_choice("███▄▄▄");
        choice_ux.set_value(0);

        let mut choice_px = menu::Choice::default()
            .with_size(100, 25)
            .below_of(&choice_ux, 5)
            .with_label("P(X)");
        choice_px.set_tooltip("Initial condition for pressure function in the pipe");
        choice_px.set_text_color(enums::Color::DarkBlue);
        choice_px.set_selection_color(enums::Color::DarkBlue);
        choice_px.add_choice("▄▄▄▄▄▄");
        choice_px.add_choice("██▄▄██");
        choice_px.add_choice("██▄▄▄▄");
        choice_px.add_choice("▄▄██▄▄");
        choice_px.add_choice("▄▄▄▄██");
        choice_px.set_value(0);

        let mut choice_left = menu::Choice::default()
            .with_size(100, 25)
            .below_of(&choice_px, 5)
            .with_label("Left side:");
        choice_left.set_tooltip("Boundary condition on the left side of the pipe");
        choice_left.add_choice("Sealed");
        choice_left.add_choice("Open");
        choice_left.set_value(0);

        let mut choice_right = menu::Choice::default()
            .with_size(100, 25)
            .below_of(&choice_left, 5)
            .with_label("Right side:");
        choice_right.set_tooltip("Boundary condition on the right side of the pipe");
        choice_right.add_choice("Sealed");
        choice_right.add_choice("Open");
        choice_right.set_value(0);

        let mut in_len = input::FloatInput::default()
            .with_size(100, 25)
            .below_of(&choice_right, 5)
            .with_label("len =");
        in_len.set_tooltip("Length of the modelled pipe");

        let mut in_n = input::IntInput::default()
            .with_size(100, 25)
            .below_of(&in_len, 5)
            .with_label("N =");
        in_n.set_tooltip("Number of nodes in the model");

        let in_a = input::FloatInput::default()
            .with_size(100, 25)
            .below_of(&in_n, 10)
            .with_label("a =");

        let in_rho = input::FloatInput::default()
            .with_size(100, 25)
            .below_of(&in_a, 5)
            .with_label("rho =");

        let in_sigma = input::FloatInput::default()
            .with_size(100, 25)
            .below_of(&in_rho, 5)
            .with_label("sigma =");

        let mut btn_apply = button::Button::default()
            .with_size(90, 25)
            .below_of(&in_sigma, 5).center_x(&g_settings)
            .with_label("Apply");
        btn_apply.set_tooltip("Apply current parameters and restart the simulation");

        g_settings.end();

        let mut g_controls = group::Group::default()
            .with_size(g_settings.w(), 90)
            .below_of(&g_settings, 30)
            .with_label("Model Controls");
        g_controls.set_frame(enums::FrameType::ShadowBox);

        let mut btn_step = button::Button::default()
            .with_size(90, 25)
            .with_pos(g_controls.x(), g_controls.y() + 15)
            .center_x(&g_controls)
            .with_label("Step");
        btn_step.set_tooltip("Make single step of the simulation");

        let mut btn_start_stop = button::Button::default()
            .with_size(90, 25)
            .below_of(&btn_step, 10)
            .with_label("Start");
        btn_start_stop.set_tooltip("Start or stop the simulation");

        g_controls.end();

        wind.end();

        app::set_focus(&btn_start_stop);

        Self {
            wind,
            uw_plot,
            pw_plot,
            choice_ux,
            choice_px,
            choice_left,
            choice_right,
            in_len,
            in_n,
            in_a,
            in_rho,
            in_sigma,
            btn_apply,
            btn_step,
            btn_start_stop
        }
    }

    pub fn show(&mut self) {
        self.wind.show();
    }

    pub fn draw_model(&mut self, m: &PipeModel) {
        self.uw_plot.draw_plot(&m.x, &m.u1, m.len);
        self.pw_plot.draw_plot(&m.x, &m.p1, m.len);
    }

    pub fn set_inputs(&mut self, m: &PipeModel) {
        let len_str = format!("{:.4}", m.len);
        self.in_len.set_value(&len_str);

        let n_str = format!("{}", m.n);
        self.in_n.set_value(&n_str);

        let a_str = format!("{:.4}", m.a);
        self.in_a.set_value(&a_str);

        let rho_str = format!("{:.4}", m.rho);
        self.in_rho.set_value(&rho_str);

        let sigma_str = format!("{:.4}", m.sigma);
        self.in_sigma.set_value(&sigma_str);

        let ui = self.choice_ux.find_index(&m.un_id);
        self.choice_ux.set_value(if ui!=-1 { ui } else { 0 });

        let pi = self.choice_px.find_index(&m.pn_id);
        self.choice_px.set_value(if pi!=-1 { pi } else { 0 });
    }

    pub fn get_inputs(&self, m: &mut PipeModel) {
        m.len = self.in_len.value().parse::<f64>().expect("Not a number!");
        m.n = self.in_n.value().parse::<usize>().expect("Not a number!");
        m.a = self.in_a.value().parse::<f64>().expect("Not a number!");
        m.rho = self.in_rho.value().parse::<f64>().expect("Not a number!");
        m.sigma = self.in_sigma.value().parse::<f64>().expect("Not a number!");

        let ux = self.choice_ux.at(self.choice_ux.value()).unwrap().label().unwrap();
        m.set_initial_u(&ux);
        println!("Type of initial conditions for velocity function: {}", ux);

        let px = self.choice_px.at(self.choice_px.value()).unwrap().label().unwrap();
        m.set_initial_p(&px);
        println!("Type of initial conditions for pressure function: {}", px);

        match self.choice_left.value() {
            0 => { m.bl = BOUNDARY_SEALED; }
            1 => { m.bl = BOUNDARY_OPEN; }
            _ => { println!("Unknown type of biundary condition!"); }
        }

        match self.choice_right.value() {
            0 => { m.br = BOUNDARY_SEALED; }
            1 => { m.br = BOUNDARY_OPEN; }
            _ => { println!("Unknown type of biundary condition!"); }
        }
    }

    pub fn set_running(&mut self, running: bool) {
        if running {
            self.choice_ux.deactivate();
            self.choice_px.deactivate();
            self.choice_left.deactivate();
            self.choice_right.deactivate();
            self.in_len.deactivate();
            self.in_n.deactivate();
            self.in_a.deactivate();
            self.in_rho.deactivate();
            self.in_sigma.deactivate();
            self.btn_apply.deactivate();
            self.btn_step.deactivate();
            self.btn_start_stop.set_label("Stop");
        }
        else {
            self.choice_ux.activate();
            self.choice_px.activate();
            self.choice_left.activate();
            self.choice_right.activate();
            self.in_len.activate();
            self.in_n.activate();
            self.in_a.activate();
            self.in_rho.activate();
            self.in_sigma.activate();
            self.btn_apply.activate();
            self.btn_step.activate();
            self.btn_start_stop.set_label("Start");
        }
    }
}
