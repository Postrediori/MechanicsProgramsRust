mod pendulum_model;
use pendulum_model::PendulumModel;

use fltk::{*, prelude::{*}};

use std::cell::RefCell;
use std::rc::Rc;
use std::{thread, time::Duration};

const THETA_0: f64 = 45.0;

// const REDRAW_DT: f64 = 0.016;
const REDRAW_DT: u64 = 16;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn main() {
    let mut model = PendulumModel::new();
    model.set_theta((THETA_0 as f64).to_radians());

    let model = Rc::from(RefCell::from(model));

    let running: bool = false;
    let running = Rc::from(RefCell::from(running));

    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    app::get_system_colors();

    let mut wind = window::Window::default()
        .with_size(WIDTH, HEIGHT).center_screen()
        .with_label("Mechanical Pendulum");

    const MODEL_SIZE: i32 = HEIGHT - 50;
    let mut model_display = frame::Frame::default()
        .with_size(MODEL_SIZE, MODEL_SIZE)
        .with_pos(25, 25);

    let offs = draw::Offscreen::new(model_display.w(), model_display.h()).unwrap();
    let offs = Rc::from(RefCell::from(offs));

    let update_display =
        |m: &PendulumModel, w: i32, h: i32, offs: &draw::Offscreen| {
            offs.begin();

            // Color palette
            const BG_COLOR: enums::Color = enums::Color::White;
            const BOUNDS_COLOR: enums::Color = enums::Color::Dark3;
            const VERT_AXIS_COLOR: enums::Color = enums::Color::Black;
            const FIX_COLOR: enums::Color = enums::Color::DarkBlue;
            const FIX_DOT_COLOR: enums::Color = enums::Color::White;
            const CORD_COLOR: enums::Color = enums::Color::Black;
            const WEIGHT_COLOR: enums::Color = enums::Color::DarkRed;

            // Clear background
            draw::draw_rect_fill(0, 0, w, h, BG_COLOR);

            // Draw bounds
            draw::set_draw_color(BOUNDS_COLOR);
            draw::set_line_style(draw::LineStyle::Solid, 1);
            draw::draw_rect(0, 0, w, h);

            // Coordinates of the pivotal point
            let x0: i32 = w / 2;
            let y0: i32 = h / 4;
            let l: i32 = h / 2;
        
            // Coordinates of the weight
            let angle: f64 = (90 as f64).to_radians() - m.theta();
            let x1: i32 = (x0 as f64 + (l as f64) * (angle).cos()) as i32;
            let y1: i32 = (y0 as f64 + (l as f64) * (angle).sin()) as i32;
        
            // Draw vertical axis
            draw::set_draw_color(VERT_AXIS_COLOR);
            draw::set_line_style(draw::LineStyle::DashDot, 1);
            draw::draw_line(x0, y0,
                x0, y0 + ((l as f64) * 1.25) as i32);

            // Draw fixation
            const FIX_WIDTH: i32 = 90;
            const FIX_HEIGHT: i32 = 25;
            draw::draw_rect_fill(x0 - FIX_WIDTH / 2, y0 - FIX_HEIGHT, FIX_WIDTH, FIX_HEIGHT, FIX_COLOR);
            
            const COLS: i32 = 10;
            const ROWS: i32 = 4;
            const SIZE: i32 = 1;
            for j in 1..ROWS {
                for i in ((j + 1) % 2)..COLS {
                    let x: i32 = x0 - FIX_WIDTH / 2 + (((i as f64 + ((j % 2) as f64) * 0.5) * (FIX_WIDTH as f64)) as i32) / COLS;
                    let y: i32 = y0 - (j * FIX_HEIGHT) / ROWS;
                    draw::draw_rect_fill(x - SIZE, y - SIZE, 2*SIZE+1, 2*SIZE+1, FIX_DOT_COLOR);
                }
            }

            // Draw cord
            draw::set_draw_color(enums::Color::Black);
            draw::set_line_style(draw::LineStyle::Solid, 2);
            draw::draw_line(x0, y0,
                x1, y1);

            // Draw weight
            const WEIGHT_RADIUS: i32 = 10;
            draw::set_draw_color(enums::Color::DarkRed);
            draw::draw_circle_fill(x1 - WEIGHT_RADIUS, y1 - WEIGHT_RADIUS, WEIGHT_RADIUS * 2, WEIGHT_COLOR);

            draw::set_line_style(draw::LineStyle::Solid, 0);

            offs.end();
        };

    model_display.draw({
        let model = model.clone();
        let offs = offs.clone();
        move |w| {
            let model = model.borrow();
            let offs = offs.borrow_mut();
            
            update_display(&model, w.w(), w.h(), &offs);

            offs.copy(w.x(), w.y(), w.w(), w.h(), 0, 0);
        }
    });

    let mut g_params = group::Group::default()
        .with_size(WIDTH - MODEL_SIZE - 75, 150)
        .right_of(&model_display, 25)
        .with_label("Model Parameters");
    g_params.set_frame(enums::FrameType::BorderFrame);
    g_params.set_color(enums::Color::Dark3);

    let mut in_g = input::Input::default()
        .with_size(90, 25)
        .with_pos(g_params.x() + 75, g_params.y() + 10)
        .with_label("g = ");
    let g_str = format!("{:.4}", model.borrow().g());
    in_g.set_value(&g_str);
    in_g.set_tooltip("Gravitational constant");

    let mut in_dtime = input::Input::default()
        .with_size(90, 25).below_of(&in_g, 10)
        .with_label("dtime = ");
    let dtime_str = format!("{:.4}", model.borrow().dtime());
    in_dtime.set_value(&dtime_str);
    in_dtime.set_tooltip("Time step delta");

    let mut in_theta0 = input::Input::default()
        .with_size(90, 25).below_of(&in_dtime, 10)
        .with_label("theta0 = ");
    let theta0_str = format!("{:.4}", model.borrow().theta().to_degrees());
    in_theta0.set_value(&theta0_str);
    in_theta0.set_tooltip("Initial pendulum angle");

    let btn_restart = button::Button::default()
        .with_size(90, 25)
        .below_of(&in_theta0, 10)
        .center_x(&g_params)
        .with_label("Apply");

    g_params.end();

    let mut g_controls = group::Group::default()
        .with_size(g_params.width(), 90)
        .below_of(&g_params, 30)
        .with_label("Model Controls");
    g_controls.set_frame(enums::FrameType::BorderFrame);
    g_controls.set_color(enums::Color::Dark3);

    let btn_step = button::Button::default()
        .with_size(90, 25)
        .with_pos(g_controls.x(), g_controls.y() + 15)
        .center_x(&g_controls)
        .with_label("Step");

    let btn_start_stop = button::Button::default()
        .with_size(90, 25)
        .below_of(&btn_step, 10)
        .with_label("Start");

    g_controls.end();

    let model_display = Rc::from(RefCell::from(model_display));
    let in_g = Rc::from(RefCell::from(in_g));
    let in_dtime = Rc::from(RefCell::from(in_dtime));
    let in_theta0 = Rc::from(RefCell::from(in_theta0));
    let btn_restart = Rc::from(RefCell::from(btn_restart));
    let btn_step = Rc::from(RefCell::from(btn_step));
    let btn_start_stop = Rc::from(RefCell::from(btn_start_stop));

    btn_restart.borrow_mut().set_callback({
        let model = model.clone();
        let model_display = model_display.clone();
        let in_g = in_g.clone();
        let in_dtime = in_dtime.clone();
        let in_theta0 = in_theta0.clone();
        move |_| {
            let mut model = model.borrow_mut();
            let mut model_display = model_display.borrow_mut();
            let in_g = in_g.borrow();
            let in_dtime = in_dtime.borrow();
            let in_theta0 = in_theta0.borrow();
            
            let g_val: f64 = in_g.value().parse::<f64>().expect("Not a number!");
            let dtime_val: f64 = in_dtime.value().parse::<f64>().expect("Not a number!");
            let theta0_val: f64 = in_theta0.value().parse::<f64>().expect("Not a number!");

            model.set_g(g_val);
            model.set_dtime(dtime_val);
            model.set_theta(theta0_val.to_radians());

            model_display.redraw();
        }
    });

    // Message to control the simulation
    #[derive(Debug, Clone, Copy)]
    pub enum Message {
        Start,
        Stop,
        Step,
        Running,
    }

    let (tx, rx) = app::channel::<Message>();

    btn_step.borrow_mut().set_callback( move |_| {
        tx.send(Message::Step);
    });

    btn_start_stop.borrow_mut().set_callback({
        let running = running.clone();
        move |_| {
            let running = running.borrow();
            if *running {
                tx.send(Message::Stop);
            }
            else {
                tx.send(Message::Start);
            }
        }
    });

    wind.end();
    wind.show();

    app::add_idle3({
        let running = running.clone();
        let model = model.clone();
        let model_display = model_display.clone();
        let in_g = in_g.clone();
        let in_dtime = in_dtime.clone();
        let in_theta0 = in_theta0.clone();
        let btn_restart = btn_restart.clone();
        let btn_step = btn_step.clone();
        let btn_start_stop = btn_start_stop.clone();
        move |_| {
            let mut running = running.borrow_mut();
            let mut model = model.borrow_mut();
            let mut model_display = model_display.borrow_mut();
            let mut in_g = in_g.borrow_mut();
            let mut in_dtime = in_dtime.borrow_mut();
            let mut in_theta0 = in_theta0.borrow_mut();
            let mut btn_restart = btn_restart.borrow_mut();
            let mut btn_step = btn_step.borrow_mut();
            let mut btn_start_stop = btn_start_stop.borrow_mut();

            if let Some(msg) = rx.recv() {
                match msg {
                    Message::Start => {
                        *running = true;

                        // Set state to running
                        in_g.deactivate();
                        in_dtime.deactivate();
                        in_theta0.deactivate();
                        btn_restart.deactivate();
                        btn_step.deactivate();
                        btn_start_stop.set_label("Stop");
                        
                        tx.send(Message::Running);
                    }
                    Message::Stop => {
                        *running = false;
                        
                        // Set state to stopped
                        in_g.activate();
                        in_dtime.activate();
                        in_theta0.activate();
                        btn_restart.activate();
                        btn_step.activate();
                        btn_start_stop.set_label("Start");
                    }
                    Message::Step => {
                        model.step();
                        model_display.redraw();
                    }
                    Message::Running => {
                        if *running {
                            // Make step and schedule next 'Running' poll
                            thread::spawn(move || {
                                tx.send(Message::Step);
                                thread::sleep(Duration::from_millis(REDRAW_DT));
                                tx.send(Message::Running);
                            });
                        }
                    }
                }
            }
        }
    });

    a.run().unwrap();
}
