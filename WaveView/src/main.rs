use fltk::{*, prelude::{*}};

use std::cell::RefCell;
use std::rc::Rc;
use std::{thread, time::Duration};

mod surface_functions;
mod wave_model;
mod wave_widget;
mod main_window;

const REDRAW_DT: u64 = 16;

fn main() {
    let running = false;

    let mut m = wave_model::WaveModel::make_model();
    m.reset();

    let a = app::App::default();

    let mut w = main_window::MainWindow::make_window();

    w.set_inputs(&m);
    w.ww.draw_model(&m);

    let running = Rc::from(RefCell::from(running));
    let m = Rc::from(RefCell::from(m));
    let w = Rc::from(RefCell::from(w));

    // Message to control the simulation
    #[derive(Debug, Clone, Copy)]
    pub enum Message {
        Start,
        Stop,
        Step,
        Running,
    }

    let (tx, rx) = app::channel::<Message>();

    w.borrow_mut().btn_apply.set_callback({
        let m = m.clone();
        let w = w.clone();
        move |_| {
            let mut m = m.borrow_mut();
            let mut w = w.borrow_mut();

            w.get_inputs(&mut m);
            m.reset();

            w.ww.draw_model(&m);
        }
    });

    w.borrow_mut().btn_step.set_callback({
        move |_| {
            tx.send(Message::Step);
        }
    });

    w.borrow_mut().btn_start_stop.set_callback({
        let running = running.clone();
        move |_|{
            let running = running.borrow();
            if *running {
                tx.send(Message::Stop);
            }
            else {
                tx.send(Message::Start);
            }
        }
    });

    app::add_idle3({
        let running = running.clone();
        let m = m.clone();
        let w = w.clone();
        move |_| {
            let mut running = running.borrow_mut();
            let mut m = m.borrow_mut();
            let mut w = w.borrow_mut();

            if let Some(msg) = rx.recv() {
                match msg {
                    Message::Start => {
                        *running = true;
                        w.set_running_status(*running);
                        tx.send(Message::Running);
                    }
                    Message::Stop => {
                        *running = false;
                        w.set_running_status(*running);
                    }
                    Message::Step => {
                        m.step();
                        w.ww.draw_model(&m);
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
