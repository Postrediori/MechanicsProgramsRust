mod frame_saver;

use fltk::{*, prelude::{*}};

use std::cell::RefCell;
use std::rc::Rc;
use std::{thread, time::Duration};

mod pipe_model;
mod plot_widget;
mod main_window;

use crate::pipe_model::PipeModel;
use crate::main_window::MainWindow;

const WIDTH: i32 = 700;
const HEIGHT: i32 = 500;

const REDRAW_DT: u64 = 16;

fn main() {
    const DEFAULT_LEN: f64 = 10.0;
    const DEFAULT_N: usize = 100;
    
    let mut model = PipeModel::new(DEFAULT_LEN, DEFAULT_N);
    model.reset();

    let a = app::App::default();
    app::get_system_colors();

    let mut w = MainWindow::make_window(WIDTH, HEIGHT,
        "Fluid mechanics in a pipe of limited length");
    w.show();

    w.set_inputs(&model);
    w.draw_model(&model);

    // Message to control the simulation
    #[derive(Debug, Clone, Copy)]
    pub enum Message {
        Start,
        Stop,
        Step,
        Running,
    }

    let (tx, rx) = app::channel::<Message>();

    let running = false;
    
    let running = Rc::from(RefCell::from(running));
    let model = Rc::from(RefCell::from(model));
    let w = Rc::from(RefCell::from(w));

    w.borrow_mut().btn_apply.set_callback({
        let model = model.clone();
        let w = w.clone();
        move |_| {
            let mut model = model.borrow_mut();
            let mut w = w.borrow_mut();

            w.get_inputs(&mut model);
            model.reset();

            w.reset_frame_counter();
            w.draw_model(&model);
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

    w.borrow_mut().btn_save_frame.set_callback({
        let w = w.clone();
        move |_|{
            let mut w = w.borrow_mut();
            w.save_frame();
        }
    });

    {
        let model = model.clone();
        let w = w.clone();
        let running = running.clone();
        while a.wait() {
            let mut model = model.borrow_mut();
            let mut w = w.borrow_mut();
            let mut running = running.borrow_mut();

            if let Some(msg) = rx.recv() {
                match msg {
                    Message::Start => {
                        *running = true;
                        w.set_running(*running);
                        tx.send(Message::Running);
                    }
                    Message::Stop => {
                        *running = false;
                        w.set_running(*running);
                    }
                    Message::Step => {
                        model.step();
                        w.draw_model(&model);
                        
                        // Save frame of the simulation
                        if w.btn_save_all_frames.value() {
                            w.save_frame();
                        }
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
    }
}
