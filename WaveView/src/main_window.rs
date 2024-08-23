use fltk::{prelude::*, *};

use crate::wave_model::WaveModel;
use crate::wave_widget::WaveWidget;

use crate::res::IconsAssets;

const WIDTH: i32 = 700;
const HEIGHT: i32 = 500;

const MARGIN: i32 = 10;
const WAVE_VIEW_SIZE: i32 = HEIGHT - MARGIN * 2;

pub struct MainWindow {
    pub ww: WaveWidget,

    surface_choice: menu::Choice,

    g_in: valuator::ValueSlider,
    h_in: valuator::ValueSlider,
    delta_in: valuator::ValueSlider,
    eps_in: valuator::ValueSlider,
    dtime_in: valuator::ValueOutput,

    pub btn_apply: button::Button,
    pub btn_step: button::Button,
    pub btn_start_stop: button::Button,
    pub btn_save_frame: button::Button,
    pub btn_save_all_frames: button::CheckButton,
}

impl MainWindow {
    pub fn make_window() -> Self {
        let mut wind = window::Window::default()
            .with_size(WIDTH, HEIGHT)
            .center_screen()
            .with_label("Wave Viewer");

        // Output widget
        let ww = WaveWidget::new(MARGIN, MARGIN, WAVE_VIEW_SIZE, WAVE_VIEW_SIZE);

        // Model params group
        let mut g_params = group::Group::default()
            .with_size(WIDTH - WAVE_VIEW_SIZE - MARGIN * 3, 230)
            .with_pos(ww.x() + ww.w() + MARGIN, 30)
            .with_label("Model parameters");
        g_params.set_frame(enums::FrameType::RoundedFrame);
        g_params.set_color(enums::Color::Black);

        let mut surface_choice = menu::Choice::default()
            .with_size(90, 25)
            .with_pos(g_params.x() + 85, g_params.y() + 10)
            .with_label("Surface :");
        surface_choice.add_choice("Linear");
        surface_choice.add_choice("Sine");
        surface_choice.add_choice("Cosine");
        surface_choice.add_choice("Halfsine");
        surface_choice.set_value(0);

        let mut g_in = valuator::ValueSlider::default()
            .with_size(125, 25)
            .below_of(&surface_choice, 5)
            .with_pos(
                g_params.x() + 50,
                surface_choice.y() + surface_choice.h() + 5,
            )
            .with_label("g =")
            .with_align(enums::Align::Left);
        g_in.set_tooltip("Gravitational constant");
        g_in.set_bounds(0.1, 20.0);
        g_in.set_type(valuator::SliderType::Horizontal);

        let mut h_in = valuator::ValueSlider::default()
            .with_size(125, 25)
            .below_of(&g_in, 5)
            .with_label("h =")
            .with_align(enums::Align::Left);
        h_in.set_bounds(0.1, 1.0);
        h_in.set_type(valuator::SliderType::Horizontal);

        let mut delta_in = valuator::ValueSlider::default()
            .with_size(125, 25)
            .below_of(&h_in, 5)
            .with_label("delta =")
            .with_align(enums::Align::Left);
        delta_in.set_bounds(0.75, 2.0);
        delta_in.set_type(valuator::SliderType::Horizontal);

        let mut eps_in = valuator::ValueSlider::default()
            .with_size(125, 25)
            .below_of(&delta_in, 5)
            .with_label("eps =")
            .with_align(enums::Align::Left);
        eps_in.set_bounds(0.01, 0.12);
        eps_in.set_type(valuator::SliderType::Horizontal);

        let mut dtime_in = valuator::ValueOutput::default()
            .with_size(90, 25)
            .with_pos(surface_choice.x(), eps_in.y() + eps_in.h() + 5)
            .with_label("dtime =")
            .with_align(enums::Align::Left);
        dtime_in.set_tooltip("Time delta of single simulation step");

        let mut btn_apply = button::Button::default()
            .with_size(90, 25)
            .below_of(&dtime_in, 10)
            .center_x(&g_params)
            .with_label("Apply");
        btn_apply.set_tooltip("Apply current parameters and restart the simulation");

        g_params.end();

        // Simulation controls group
        let mut g_controls = group::Group::default()
            .with_size(g_params.w(), 90)
            .below_of(&g_params, 30)
            .with_label("Model Controls");
        g_controls.set_frame(enums::FrameType::RoundedFrame);
        g_controls.set_color(enums::Color::Black);

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

        let mut g_capture = group::Group::default()
            .with_size(g_params.w(), 75)
            .below_of(&g_controls, 15);
        g_capture.set_frame(enums::FrameType::RoundedFrame);
        g_capture.set_color(enums::Color::Black);

        let mut btn_save_frame = button::Button::default()
            .with_size(90, 25)
            .with_pos(g_capture.x(), g_capture.y() + 10)
            .center_x(&g_capture)
            .with_label("Save frame");
        btn_save_frame.set_tooltip("Save single frame of the simulation");

        let mut btn_save_all_frames = button::CheckButton::default()
            .with_size(90, 25)
            .with_pos(
                g_capture.x() + 35,
                btn_save_frame.y() + btn_save_frame.h() + 5,
            )
            .with_label("Save all frames");
        btn_save_all_frames.set_tooltip(
            "Save all frames of the running simulation (this will slow down the program a lot!)",
        );

        g_capture.end();

        if let Some(img) = IconsAssets::get("WaveView32.png") {
            if let Ok(img) = fltk::image::PngImage::from_data(img.data.as_ref()) {
                wind.set_icon(Some(img));
            }
        }

        wind.end();
        wind.show();

        app::set_focus(&btn_start_stop);

        Self {
            ww,
            surface_choice,
            g_in,
            h_in,
            delta_in,
            eps_in,
            dtime_in,
            btn_apply,
            btn_step,
            btn_start_stop,
            btn_save_frame,
            btn_save_all_frames,
        }
    }

    // Copy inputs from the parameters of the model
    pub fn set_inputs(&mut self, m: &WaveModel) {
        self.g_in.set_value(m.g);
        self.h_in.set_value(m.h);
        self.delta_in.set_value(m.delta);
        self.eps_in.set_value(m.epsilon);
        self.dtime_in.set_value(m.dtime);
    }

    // Copy parameters from UI to the model
    pub fn get_inputs(&self, m: &mut WaveModel) {
        m.set_surface_func(self.surface_choice.value());
        m.g = self.g_in.value();
        m.h = self.h_in.value();
        m.delta = self.delta_in.value();
        m.epsilon = self.eps_in.value();
    }

    pub fn set_running_status(&mut self, running: bool) {
        if running {
            self.surface_choice.deactivate();
            self.g_in.deactivate();
            self.h_in.deactivate();
            self.delta_in.deactivate();
            self.eps_in.deactivate();
            self.btn_apply.deactivate();
            self.btn_step.deactivate();
            self.btn_start_stop.set_label("Stop");
        } else {
            self.surface_choice.activate();
            self.g_in.activate();
            self.h_in.activate();
            self.delta_in.activate();
            self.eps_in.activate();
            self.btn_apply.activate();
            self.btn_step.activate();
            self.btn_start_stop.set_label("Start");
        }
    }
}
