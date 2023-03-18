mod pendulum_model;

use fltk::{*, prelude::{*}};

use std::cell::RefCell;
use std::rc::Rc;

const THETA_0: f64 = 45.0;

const REDRAW_DT: f64 = 0.016;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn main() {
    let mut model = pendulum_model::PendulumModel::new();
    model.set_theta((THETA_0 as f64).to_radians());

    let model = Rc::from(RefCell::from(model));

    let running: bool = false;
    let running = Rc::from(RefCell::from(running));

    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    app::get_system_colors();

    let mut wind = window::Window::default()
        .with_size(WIDTH, HEIGHT).center_screen()
        .with_label("Mechanical Pendulum");

    let mut model_display = frame::Frame::default()
        .with_size(HEIGHT - 50, HEIGHT - 50)
        .with_pos(25, 25);

    model_display.draw({
        let model = model.clone();
        move |w| {
            let model = model.borrow();
            
            // Clear widget
            draw::draw_rect_fill(w.x(), w.y(), w.w(), w.h(), enums::Color::White);

            draw::set_draw_color(enums::Color::Black);
            draw::draw_rect(w.x(), w.y(), w.w(), w.h());

            // Coordinates of the pivotal point
            let x0: i32 = w.x() + w.w() / 2;
            let y0: i32 = w.y() + w.h() / 4;
            let l: i32 = w.h() / 2;
        
            // Coordinates of the weight
            let angle: f64 = (90 as f64).to_radians() - model.theta();
            let x1: i32 = (x0 as f64 + (l as f64) * (angle).cos()) as i32;
            let y1: i32 = (y0 as f64 + (l as f64) * (angle).sin()) as i32;
        
            // Draw vertical axis
            draw::set_draw_color(enums::Color::Black);
            draw::set_line_style(draw::LineStyle::DashDot, 1);
            draw::draw_line(x0, y0,
                x0, y0 + ((l as f64) * 1.25) as i32);

            // Draw fixation
            const FIX_WIDTH: i32 = 90;
            const FIX_HEIGHT: i32 = 25;
            draw::draw_rect_fill(x0 - FIX_WIDTH / 2, y0 - FIX_HEIGHT, FIX_WIDTH, FIX_HEIGHT, enums::Color::DarkBlue);
            
            const COLS: i32 = 10;
            const ROWS: i32 = 4;
            const SIZE: i32 = 1;
            for j in 1..ROWS {
                for i in ((j + 1) % 2)..COLS {
                    let x: i32 = x0 - FIX_WIDTH / 2 + (((i as f64 + ((j % 2) as f64) * 0.5) * (FIX_WIDTH as f64)) as i32) / COLS;
                    let y: i32 = y0 - (j * FIX_HEIGHT) / ROWS;
                    draw::draw_rect_fill(x - SIZE, y - SIZE, 2*SIZE+1, 2*SIZE+1, enums::Color::White);
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
            draw::draw_circle_fill(x1 - WEIGHT_RADIUS, y1 - WEIGHT_RADIUS, WEIGHT_RADIUS * 2, enums::Color::DarkRed);
        }
    });

    let mut in_g = input::Input::default()
        .with_size(90, 25).right_of(&model_display, 80)
        .with_label("g = ");
    let g_str = format!("{:.4}", model.borrow().g());
    in_g.set_value(&g_str);

    let mut in_dtime = input::Input::default()
        .with_size(90, 25).below_of(&in_g, 10)
        .with_label("dtime = ");
    let dtime_str = format!("{:.4}", model.borrow().dtime());
    in_dtime.set_value(&dtime_str);

    let mut in_theta0 = input::Input::default()
        .with_size(90, 25).below_of(&in_dtime, 10)
        .with_label("theta0 = ");
    let theta0_str = format!("{:.4}", model.borrow().theta().to_degrees());
    in_theta0.set_value(&theta0_str);

    let btn_restart = button::Button::default()
        .with_size(90, 25).below_of(&in_theta0, 10)
        .with_label("Restart");

    let btn_step = button::Button::default()
        .with_size(90, 25).below_of(&btn_restart, 10)
        .with_label("Step");

    let btn_start = button::Button::default()
        .with_size(90, 25).below_of(&btn_step, 10)
        .with_label("Start");

    let mut btn_stop = button::Button::default()
        .with_size(90, 25).below_of(&btn_start, 10)
        .with_label("Stop");
    btn_stop.deactivate();

    let model_display = Rc::from(RefCell::from(model_display));
    let in_g = Rc::from(RefCell::from(in_g));
    let in_dtime = Rc::from(RefCell::from(in_dtime));
    let in_theta0 = Rc::from(RefCell::from(in_theta0));
    let btn_restart = Rc::from(RefCell::from(btn_restart));
    let btn_step = Rc::from(RefCell::from(btn_step));
    let btn_start = Rc::from(RefCell::from(btn_start));
    let btn_stop = Rc::from(RefCell::from(btn_stop));

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

    btn_step.borrow_mut().set_callback({
        let model = model.clone();
        let model_display = model_display.clone();
        move |_| {
            let mut model = model.borrow_mut();
            let mut model_display = model_display.borrow_mut();

            model.step();

            model_display.redraw();
        }
    });

    btn_start.borrow_mut().set_callback({
        let running = running.clone();
        let in_g = in_g.clone();
        let in_dtime = in_dtime.clone();
        let in_theta0 = in_theta0.clone();
        let btn_restart = btn_restart.clone();
        let btn_step = btn_step.clone();
        let btn_start = btn_start.clone();
        let btn_stop = btn_stop.clone();
        move |_| {
            let mut running = running.borrow_mut();
            let mut in_g = in_g.borrow_mut();
            let mut in_dtime = in_dtime.borrow_mut();
            let mut in_theta0 = in_theta0.borrow_mut();
            let mut btn_restart = btn_restart.borrow_mut();
            let mut btn_step = btn_step.borrow_mut();
            let mut btn_start = btn_start.borrow_mut();
            let mut btn_stop = btn_stop.borrow_mut();

            *running = true;

            in_g.deactivate();
            in_dtime.deactivate();
            in_theta0.deactivate();
            btn_restart.deactivate();
            btn_step.deactivate();
            btn_start.deactivate();
            btn_stop.activate();
        }
    });

    btn_stop.borrow_mut().set_callback({
        let running = running.clone();
        let in_g = in_g.clone();
        let in_dtime = in_dtime.clone();
        let in_theta0 = in_theta0.clone();
        let btn_restart = btn_restart.clone();
        let btn_step = btn_step.clone();
        let btn_start = btn_start.clone();
        let btn_stop = btn_stop.clone();
        move |_| {
            let mut running = running.borrow_mut();
            let mut in_g = in_g.borrow_mut();
            let mut in_dtime = in_dtime.borrow_mut();
            let mut in_theta0 = in_theta0.borrow_mut();
            let mut btn_restart = btn_restart.borrow_mut();
            let mut btn_step = btn_step.borrow_mut();
            let mut btn_start = btn_start.borrow_mut();
            let mut btn_stop = btn_stop.borrow_mut();

            *running = false;

            in_g.activate();
            in_dtime.activate();
            in_theta0.activate();
            btn_restart.activate();
            btn_step.activate();
            btn_start.activate();
            btn_stop.deactivate();
        }
    });

    wind.end();
    wind.show();

    app::add_idle3({
        let running = running.clone();
        let model = model.clone();
        let model_display = model_display.clone();
        move |_| {
            let running = running.borrow();
            let mut model = model.borrow_mut();
            let mut model_display = model_display.borrow_mut();
            if (*running) {
                model.step();

                model_display.redraw();

                app::sleep(REDRAW_DT);
            }
        }
    });

    a.run().unwrap();
}
