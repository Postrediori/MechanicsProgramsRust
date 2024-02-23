mod draw_primitives;
mod param_list;
mod pendulum_model;
mod simple_pendulum;
mod elastic_pendulum;
mod coupled_pendulums;
mod double_pendulum;
mod param_table_widget;

use param_list::{ParamList, Parametrized};
use pendulum_model::{PendulumModel, ParametrizedModel};
use simple_pendulum::SimplePendulumModel;
use elastic_pendulum::ElasticPendulumModel;
use coupled_pendulums::CoupledPendulumsModel;
use double_pendulum::DoublePendulumModel;
use param_table_widget::ParamTableWidget;

use fltk::{*, prelude::*};

use std::cell::RefCell;
use std::rc::Rc;
use std::{thread, time::Duration};


const REDRAW_DT: u64 = 16;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;
const MARGIN: i32 = 10;
const MODEL_WIDGET_SIZE: i32 = HEIGHT - MARGIN*2;

// Message to control the simulation
#[derive(Debug, Clone, Copy)]
pub enum Message {
    Apply,
    StartStop,
    Start,
    Stop,
    Step,
    Running,
    SelectModel(usize),
}

struct ModelList {
    current_model: usize,
    models: Vec<Box<dyn ParametrizedModel>>,
}

impl ModelList {
    // fn current_model(&self) -> usize {
    //     self.current_model
    // }
    fn set_current_model(&mut self, n: usize) {
        self.current_model = n;
    }
}

impl PendulumModel for ModelList {
    fn label(&self) -> &'static str { self.models[self.current_model].label() }
    fn time(&self) -> f64 { self.models[self.current_model].time() }
    fn restart(&mut self) { self.models[self.current_model].restart(); }
    fn step(&mut self) { self.models[self.current_model].step(); }
    fn draw(&self, w: i32, h: i32, offs: &draw::Offscreen) {
        self.models[self.current_model].draw(w, h, offs);
    }
}

impl Parametrized for ModelList {
    fn copy_params_from(&mut self, other: &ParamList) {
        self.models[self.current_model].copy_params_from(other);
    }
    fn get_params(&self) -> ParamList {
        self.models[self.current_model].get_params()
    }
}

fn main() {
    // Models list object
    let models = ModelList {
        current_model: 0,
        models: vec![
            Box::from(SimplePendulumModel::new()),
            Box::from(ElasticPendulumModel::new()),
            Box::from(CoupledPendulumsModel::new()),
            Box::from(DoublePendulumModel::new()),
        ],
    };

    let models = Rc::from(RefCell::from(models));

    models.borrow_mut().restart();

    // Create app
    let mut running: bool = false;

    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    app::get_system_colors();

    let (tx, rx) = app::channel::<Message>();

    // Create window
    let mut wind = window::Window::default()
        .with_size(WIDTH, HEIGHT).center_screen()
        .with_label("Mechanical Pendulum");

    let mut main_layout = group::Flex::default_fill().row();
    main_layout.set_margin(MARGIN);

    const MODEL_SIZE: i32 = HEIGHT - MARGIN*2;
    let mut model_widget = frame::Frame::default()
        .with_size(MODEL_SIZE, MODEL_SIZE);

    // Controls panel
    let mut controls_column = group::Flex::default_fill().column();
    controls_column.set_margin(MARGIN);

    // Spacer
    let mut spacer = frame::Frame::default();
    controls_column.set_size(&mut spacer, 15);

    // Model selector
    let mut model_select_group;
    {
        model_select_group = group::Flex::default_fill().column()
            .with_label("Pendulum model");
        model_select_group.set_frame(enums::FrameType::BorderFrame);
        model_select_group.set_color(enums::Color::Dark3);
        model_select_group.set_margin(5);

        for i in 0..models.borrow().models.len() {
            let mut btn = button::RadioRoundButton::default()
                .with_label(models.borrow().models[i].label());
            btn.emit(tx, Message::SelectModel(i));
            if i==0 {
                btn.set_value(true);
            }

            model_select_group.set_size(&mut btn, 25);
        }

        model_select_group.end();

        controls_column.set_size(&mut model_select_group, models.borrow().models.len() as i32 * 30 + 5);
    }

    // Spacer
    let mut spacer = frame::Frame::default();
    controls_column.set_size(&mut spacer, 15);

    // Parameters section
    let mut table;
    let mut apply_btn;
    {
        let mut params_group = group::Flex::default_fill().column()
            .with_label("Model Parameters");
        params_group.set_frame(enums::FrameType::BorderFrame);
        params_group.set_color(enums::Color::Dark3);
        params_group.set_margin(5);
    
        table = ParamTableWidget::new();
        table.set_size_in_flex(&mut params_group, 140);

        {
            let mut row = group::Flex::default_fill().row();

            frame::Frame::default();

            apply_btn = button::Button::default()
                .with_label("@refresh Apply @refresh");
            apply_btn.set_tooltip("Apply parameters and restart the model");
            apply_btn.emit(tx, Message::Apply);
            row.set_size(&mut apply_btn, 90);

            frame::Frame::default();

            row.end();
            params_group.set_size(&mut row, 25);
        }

        params_group.end();

        controls_column.set_size(&mut params_group, 185);
    }

    // Spacer
    let mut spacer = frame::Frame::default();
    controls_column.set_size(&mut spacer, 15);

    // Model controls
    let mut step_btn;
    let mut start_stop_btn;
    {
        let mut group = group::Flex::default_fill().column()
            .with_label("Model Controls");
        group.set_frame(enums::FrameType::BorderFrame);
        group.set_color(enums::Color::Dark3);
        group.set_margin(5);

        {
            let mut row = group::Flex::default_fill().row();

            frame::Frame::default();

            step_btn = button::Button::default()
                .with_size(90, 25)
                .with_label("@>| Step @>|");
            step_btn.set_tooltip("Perform one step of the simulation");
            step_btn.emit(tx, Message::Step);
            row.set_size(&mut step_btn, 90);

            frame::Frame::default();

            row.end();
            group.set_size(&mut row, 25);
        }

        {
            let mut row = group::Flex::default_fill().row();

            frame::Frame::default();

            start_stop_btn = button::Button::default()
                .with_label("@> Start @>");
            start_stop_btn.set_tooltip("Start/Stop the simulation");
            start_stop_btn.emit(tx, Message::StartStop);
            row.set_size(&mut start_stop_btn, 90);

            app::set_focus(&start_stop_btn);

            frame::Frame::default();

            row.end();
            group.set_size(&mut row, 25);
        }

        group.end();
        controls_column.set_size(&mut group, 65);
    }

    controls_column.end();
    main_layout.set_size(&mut controls_column, WIDTH-MODEL_WIDGET_SIZE-MARGIN*3);

    main_layout.end();

    wind.end();

    let offs = draw::Offscreen::new(model_widget.w(), model_widget.h()).unwrap();
    let offs = Rc::from(RefCell::from(offs));

    model_widget.draw({
        let models = models.clone();
        let offs = offs.clone();
        move |w| {
            let models = models.borrow_mut();
            let offs = offs.borrow_mut();
            
            models.draw(w.w(), w.h(), &offs);
            
            offs.copy(w.x(), w.y(), w.w(), w.h(), 0, 0);
        }
    });

    wind.show();

    // Initial setup
    table.copy_params_from(&models.borrow().get_params());

    // Main loop
    while a.wait() {
        if let Some(msg) = rx.recv() {
            match msg {
                Message::Apply => {
                    models.borrow_mut().copy_params_from(&table.get_params());
        
                    models.borrow_mut().restart();
        
                    model_widget.redraw();
                }
                Message::StartStop => {
                    tx.send(if running { Message::Stop } else { Message::Start });
                }
                Message::Start => {
                    running = true;

                    // Set state to running
                    model_select_group.deactivate();
                    apply_btn.deactivate();
                    table.deactivate();
                    step_btn.deactivate();
                    start_stop_btn.set_label("@|| Stop @||");
                    
                    tx.send(Message::Running);
                }
                Message::Stop => {
                    running = false;
                    
                    // Set state to stopped
                    model_select_group.activate();
                    apply_btn.activate();
                    table.activate();
                    step_btn.activate();
                    start_stop_btn.set_label("@> Start @>");
                }
                Message::Step => {
                    models.borrow_mut().step();
                    model_widget.redraw();
                }
                Message::Running => {
                    if running {
                        // Make step and schedule next 'Running' poll
                        thread::spawn(move || {
                            tx.send(Message::Step);
                            thread::sleep(Duration::from_millis(REDRAW_DT));
                            tx.send(Message::Running);
                        });
                    }
                }
                Message::SelectModel(k) => {
                    models.borrow_mut().set_current_model(k);
                    models.borrow_mut().restart();
                    table.copy_params_from(&models.borrow().get_params());
                    model_widget.redraw();
                }
            }
        }
    }
}
