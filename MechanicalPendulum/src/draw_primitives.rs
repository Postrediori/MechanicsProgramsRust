use std::f64::consts::TAU;

use fltk::{enums, draw};

/*
 * Colors
 */
const AXIS_COLOR: enums::Color = enums::Color::Black;

const REST_COLOR: enums::Color = enums::Color::DarkBlue;
const REST_DOT_COLOR: enums::Color = enums::Color::White;
const CORD_COLOR: enums::Color = enums::Color::Black;
const SPRING_COLOR: enums::Color = enums::Color::Black;
const WEIGHT_COLOR: enums::Color = enums::Color::DarkRed;


/*
 * Primitive drawing functions
 */

pub fn draw_axis(x1: i32, y1: i32, x2: i32, y2: i32) {
    const AXIS_WIDTH: i32 = 1;

    draw::set_draw_color(AXIS_COLOR);
    draw::set_line_style(draw::LineStyle::DashDot, AXIS_WIDTH);

    draw::draw_line(x1, y1, x2, y2);
}

pub fn draw_cord(x1: i32, y1: i32, x2: i32, y2: i32) {
    const CORD_WIDTH: i32 = 2;

    draw::set_draw_color(CORD_COLOR);
    draw::set_line_style(draw::LineStyle::Solid, CORD_WIDTH);

    draw::draw_line(x1, y1, x2, y2);
}

pub fn draw_spring(x1: i32, y1: i32, x2: i32, y2: i32, sections: i32, width: i32) {
    draw::set_draw_color(SPRING_COLOR);
    draw::set_line_style(draw::LineStyle::Solid, 2);

    // Number of sections plus beginning and ending segments
    let n = sections + 2;

    // Geometric parameters
    let dx = (x2 - x1) as f64;
    let dy = (y2 - y1) as f64;

    let length = (dx * dx + dy * dy).sqrt();
    let alpha = dy.atan2(dx);

    // Length of a single segment (also beginnign and ending segments)
    let dl = length / (n as f64);

    // Draw spring
    let k = sections * 12;
    let l_spring = length - dl * 2.0;
    let dl_spring = l_spring / (k as f64);

    draw::begin_line();

    // Draw beginning segment
    draw::vertex(x1 as f64, y1 as f64);

    for i in 0..k+1 {
        let x_l = dl + dl_spring * (i as f64); // +dl for 'real' beginning of spring segments
        let y_l = ((i as f64) * (sections as f64) * TAU / (k as f64)).sin() * (width as f64);

        // Rotation around (x1,y1)
        let x = alpha.cos() * x_l - alpha.sin() * y_l;
        let y = alpha.sin() * x_l + alpha.cos() * y_l;

        draw::vertex(x1 as f64 + x, y1 as f64 + y);
    }

    // Draw ending segment
    draw::vertex(x2 as f64, y2 as f64);
    
    draw::end_line();
}

pub fn draw_rest(x: i32, y: i32, width: i32, height: i32) {
    let x0 = x - width / 2;
    let y0 = y - height / 2;
    draw::draw_rect_fill(x0, y0, width, height, REST_COLOR);
    
    const COLS: i32 = 10;
    const ROWS: i32 = 4;
    const SIZE: i32 = 1;
    draw::set_draw_color(REST_DOT_COLOR);
    for j in 1..ROWS {
        for i in ((j + 1) % 2)..COLS {
            let x: i32 = x0 + (((i as f64 + ((j % 2) as f64) * 0.5) * (width as f64)) as i32) / COLS;
            let y: i32 = y0 + (j * height) / ROWS;
            draw::draw_circle_fill(x - SIZE, y - SIZE, 2*SIZE+1, REST_DOT_COLOR);
        }
    }
}

pub fn draw_weight(x: i32, y: i32) {
    const WEIGHT_RADIUS: i32 = 10;
    
    draw::set_draw_color(WEIGHT_COLOR);
    draw::draw_circle_fill(x - WEIGHT_RADIUS, y - WEIGHT_RADIUS, WEIGHT_RADIUS * 2, WEIGHT_COLOR);
}
