#![allow(clippy::too_many_lines)]

mod bessel_func;
mod plot_widget;
use plot_widget::{Area, PlotFunctionInfo, PlotLines, PlotWidget};

mod res;
use res::IconsAssets;

use fltk::{app, button, enums, frame, group, input, prelude::*, window};

use std::thread;

const WIDTH: i32 = 700;
const HEIGHT: i32 = 500;
const MARGIN: i32 = 10;

const LINE_COUNT: i32 = 500;

enum Message {
    UpdateArea,
    UpdatePlots(Vec<PlotFunctionInfo>),
    FinishCalculation((PlotFunctionInfo, PlotLines)),
}

fn main() {
    // Plot parameters
    const DEFAULT_AREA: Area = Area {
        xmin: 0.0,
        xmax: 20.0,
        ymin: -3.0,
        ymax: 1.0,
    };

    let plots: Vec<PlotFunctionInfo> = [
        PlotFunctionInfo {
            f: bessel_func::y0_1,
            color: enums::Color::from_u32(0x00_a0_00_00),
            name: "Integration".to_string(),
        },
        PlotFunctionInfo {
            f: bessel_func::y0_2,
            color: enums::Color::from_u32(0x00_99_cc_ff),
            name: "Infinite series".to_string(),
        },
    ]
    .to_vec();

    // App and main window
    let a = app::App::default();
    app::get_system_colors();

    let mut wind = window::Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label("Bessel Function");
    wind.make_resizable(true);

    let mut main_layout = group::Flex::default_fill().row();
    main_layout.set_margin(MARGIN);
    main_layout.set_spacing(MARGIN);

    // Instruments column
    let mut controls_column = group::Flex::default_fill().column();

    {
        let spacer = frame::Frame::default();
        controls_column.fixed(&spacer, 5);
    }

    // Plot bounds
    let mut bounds_frame = group::Group::default()
        .with_size(100, 100)
        .with_pos(10, 0)
        .with_label("Plot bounds");
    bounds_frame.set_frame(enums::FrameType::BorderFrame);
    bounds_frame.set_color(enums::Color::Black);

    let mut bounds_frame_flex = group::Flex::default_fill().column();
    bounds_frame_flex.set_margin(MARGIN);

    {
        let spacer = frame::Frame::default();
        bounds_frame_flex.fixed(&spacer, 5);
    }

    let mut in_max_y;
    {
        let mut row = group::Flex::default_fill().row();

        frame::Frame::default();

        in_max_y = input::FloatInput::default().with_label("y max");
        in_max_y.set_align(enums::Align::Top);
        row.fixed(&in_max_y, 75);

        frame::Frame::default();

        row.end();

        bounds_frame_flex.fixed(&row, 25);
    }

    let mut in_min_x;
    let mut in_max_x;
    {
        let mut row = group::Flex::default_fill().row();

        frame::Frame::default();

        in_min_x = input::FloatInput::default().with_label("x min");
        in_min_x.set_align(enums::Align::Left);
        row.fixed(&in_min_x, 45);

        in_max_x = input::FloatInput::default().with_label("x max");
        in_max_x.set_align(enums::Align::Right);
        row.fixed(&in_max_x, 45);

        frame::Frame::default();

        row.end();

        bounds_frame_flex.fixed(&row, 25);
    }

    let mut in_min_y;
    {
        let mut row = group::Flex::default_fill().row();

        frame::Frame::default();

        in_min_y = input::FloatInput::default().with_label("y min");
        in_min_y.set_align(enums::Align::Bottom);
        row.fixed(&in_min_y, 75);

        frame::Frame::default();

        row.end();

        bounds_frame_flex.fixed(&row, 25);
    }

    {
        let spacer = frame::Frame::default();
        bounds_frame_flex.fixed(&spacer, 15);
    }

    let mut btn_redraw;
    {
        let mut row = group::Flex::default_fill().row();

        frame::Frame::default();

        btn_redraw = button::Button::default().with_label("Update");
        row.fixed(&btn_redraw, 90);

        frame::Frame::default();

        row.end();

        bounds_frame_flex.fixed(&row, 25);
    }

    bounds_frame_flex.end();

    bounds_frame.end();
    // controls_column.fixed(&bounds_frame, 230);

    {
        let spacer = frame::Frame::default();
        controls_column.fixed(&spacer, 10);
    }

    // Legend
    let mut legend_frame = group::Group::default()
        .with_pos(0, 0)
        .with_size(100, 100)
        .with_label("Legend");
    legend_frame.set_frame(enums::FrameType::BorderFrame);
    legend_frame.set_color(enums::Color::Black);

    let mut pack = group::Flex::default()
        .column()
        .with_pos(legend_frame.x(), legend_frame.y())
        .with_size(legend_frame.w(), legend_frame.h());
    pack.set_spacing(MARGIN);
    pack.set_margin(MARGIN);

    // controls_column.fixed(&legend_frame, 200);

    pack.end();

    legend_frame.end();

    controls_column.end();
    main_layout.fixed(&controls_column, 190);

    // Plot widget column
    let mut plot_widget = PlotWidget::new(0, 0, 100, 100);

    main_layout.end();

    if let Some(img) = IconsAssets::get("BesselGraph32.png") {
        if let Ok(img) = fltk::image::PngImage::from_data(img.data.as_ref()) {
            wind.set_icon(Some(img));
        }
    }

    wind.end();

    let (tx, rx) = app::channel::<Message>();

    // Callbacks
    in_max_x.set_callback({
        let tx = tx.clone();
        move |_b| {
            tx.send(Message::UpdateArea);
        }
    });
    in_min_x.set_callback({
        let tx = tx.clone();
        move |_b| {
            tx.send(Message::UpdateArea);
        }
    });
    in_max_y.set_callback({
        let tx = tx.clone();
        move |_b| {
            tx.send(Message::UpdateArea);
        }
    });
    in_min_y.set_callback({
        let tx = tx.clone();
        move |_b| {
            tx.send(Message::UpdateArea);
        }
    });

    btn_redraw.set_callback({
        let tx = tx.clone();
        move |_b| {
            tx.send(Message::UpdateArea);
        }
    });

    // Initial setup
    {
        let area = DEFAULT_AREA;

        plot_widget.set_area(area);

        in_max_x.set_value(format!("{:.1}", area.xmax).as_str());
        in_min_x.set_value(format!("{:.1}", area.xmin).as_str());
        in_max_y.set_value(format!("{:.1}", area.ymax).as_str());
        in_min_y.set_value(format!("{:.1}", area.ymin).as_str());
    }

    // Setup plot legend
    let mut legend: Vec<frame::Frame> = vec![];
    {
        pack.begin();
        for p in &plots {
            let mut row = group::Flex::default().row();

            let mut legend_color = frame::Frame::default();
            legend_color.set_frame(enums::FrameType::FlatBox);
            legend_color.set_color(enums::Color::lighter(&p.color));
            row.fixed(&legend_color, 45);

            let mut legend_name = frame::Frame::default().with_label(&p.name);
            legend_name.set_label_color(enums::Color::Dark3);
            legend.push(legend_name);

            row.end();
            pack.fixed(&row, 30);
        }
        pack.end();
    }

    wind.show();

    // Calculate functions
    tx.send(Message::UpdatePlots(plots.clone()));

    while a.wait() {
        if let Some(msg) = rx.recv() {
            match msg {
                Message::UpdateArea => {
                    let max_x: f64 = in_max_x.value().parse::<f64>().expect("Not a number!");
                    let min_x: f64 = in_min_x.value().parse::<f64>().expect("Not a number!");
                    let max_y: f64 = in_max_y.value().parse::<f64>().expect("Not a number!");
                    let min_y: f64 = in_min_y.value().parse::<f64>().expect("Not a number!");

                    plot_widget.set_area(plot_widget::Area {
                        xmin: min_x.max(0.0),
                        xmax: max_x,
                        ymin: min_y,
                        ymax: max_y,
                    });

                    tx.send(Message::UpdatePlots(plots.clone()));
                }
                Message::UpdatePlots(plots) => {
                    plot_widget.clear_plots();
                    plot_widget.redraw();

                    for w in &mut legend {
                        w.set_label_color(enums::Color::Dark3);
                        w.redraw_label();
                    }

                    let area = plot_widget.get_area();
                    let _ = plots
                        .into_iter()
                        .map(|p| {
                            thread::spawn({
                                let tx = tx.clone();
                                move || {
                                    println!("Start calculation of plot {}", p.name);

                                    // Calculate plot points
                                    let lines_vec = p.calc_points(LINE_COUNT, &area);
                                    tx.send(Message::FinishCalculation((p, lines_vec)));
                                }
                            });
                        })
                        .collect::<Vec<_>>();
                }
                Message::FinishCalculation(p) => {
                    println!("Calculated plot {} with {} lines", &p.0.name, p.1.len());

                    if let Some(w) = legend.iter_mut().find(|w| w.label() == p.0.name) {
                        w.set_label_color(p.0.color.darker());
                        w.redraw_label();
                    }

                    plot_widget.add_plot(p);
                    plot_widget.redraw();
                }
            }
        }
    }
}
