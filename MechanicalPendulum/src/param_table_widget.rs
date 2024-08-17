use fltk::{prelude::*, *};

use crate::param_list::*;

use std::cell::RefCell;
use std::rc::Rc;

const COLUMN_WIDTH: i32 = 68;

struct ParamTableWidgetInner {
    table: table::Table,
    input: input::FloatInput,
    params: ParamList,
    edit_cell: Option<(i32, i32)>,
}

impl ParamTableWidgetInner {
    fn start_editing(&mut self, row: i32, col: i32) {
        self.edit_cell = Some((row, col));
    }

    fn finish_editing(&mut self) {
        if let Some(edit_cell) = self.edit_cell {
            let str = self.input.value();
            match str.parse::<f64>() {
                Ok(val) => self.params.set(edit_cell.0 as usize, val),
                Err(_) => eprintln!("Not a number!"),
            };
            self.edit_cell = None;
            self.input.hide();

            // Prevent mouse cursor from remaining hidden after pressing Enter.
            self.input
                .window()
                .unwrap()
                .set_cursor(enums::Cursor::Default);
        }
    }

    fn draw_cell(
        &mut self,
        ctx: table::TableContext,
        row: i32,
        col: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        match ctx {
            table::TableContext::StartPage => draw::set_font(enums::Font::Helvetica, 14),
            table::TableContext::ColHeader => {
                // Column titles
                draw_header("Value", x, y, width, height)
            }
            table::TableContext::RowHeader => {
                // Row titles
                draw_header(&self.params.get_title(row as usize), x, y, width, height);
            }
            table::TableContext::Cell => {
                let coord = (row, col);
                if self.edit_cell == Some(coord) {
                    self.input.resize(x, y, width, height);
                    self.input.show();

                    self.input
                        .set_value(&format!("{:.4}", self.params.get(row as usize)));
                    self.input
                        .set_tooltip(&self.params.get_tooltip(row as usize));
                    self.input.take_focus().expect("input refused focus");
                    self.input.redraw();
                } else {
                    // Data in cells
                    draw_data(
                        &format!("{:.4}", self.params.get(row as usize)),
                        x,
                        y,
                        width,
                        height,
                        if !self.table.active() {
                            CellState::Deactivated
                        } else {
                            if self.table.is_selected(row, col) {
                                CellState::Selected
                            } else {
                                CellState::Normal
                            }
                        },
                    );
                }
            }
            _ => (),
        }
    }
}

pub struct ParamTableWidget {
    inner: Rc<RefCell<ParamTableWidgetInner>>,
}

impl ParamTableWidget {
    pub fn new(/*flex: &mut group::Flex, height: i32*/) -> Self {
        let mut table = table::Table::default();
        table.end();
        table.set_rows(0);
        table.set_row_header(true);
        table.set_cols(1);
        table.set_col_header(true);
        table.set_col_width(0, COLUMN_WIDTH);
        table.set_row_header_width(COLUMN_WIDTH);

        let mut cell_input = input::FloatInput::default();
        cell_input.set_trigger(enums::CallbackTrigger::EnterKeyAlways);
        cell_input.hide();

        let inner = Rc::new(RefCell::new(ParamTableWidgetInner {
            table,
            input: cell_input,
            params: ParamList::new(),
            edit_cell: None,
        }));

        let inner_clone = inner.clone();
        inner.borrow_mut().input.set_callback(move |_| {
            let mut inner = inner_clone.borrow_mut();
            inner.finish_editing();
        });

        let inner_clone = inner.clone();
        inner
            .borrow_mut()
            .table
            .draw_cell(move |_, ctx, row, col, x, y, width, height| {
                match inner_clone.try_borrow_mut() {
                    Ok(mut inner) => {
                        inner.draw_cell(ctx, row, col, x, y, width, height);
                    }
                    Err(error) => {
                        eprintln!("Error in table.draw_cell: {}", error);
                    }
                }
            });

        inner.borrow_mut().table.handle({
            let inner_clone = inner.clone();
            move |widget, event| match inner_clone.try_borrow_mut() {
                Ok(mut inner) => {
                    if event == enums::Event::Push
                        && widget.callback_context() == table::TableContext::Cell
                    {
                        inner.finish_editing();
                        if app::event_clicks() {
                            inner.start_editing(widget.callback_row(), widget.callback_col());
                        }
                        widget.redraw();
                        true
                    } else {
                        false
                    }
                }
                Err(error) => {
                    eprintln!("Error in table.handle: {}", error);
                    false
                }
            }
        });

        Self { inner }
    }

    pub fn set_size_in_flex(&mut self, flex: &mut group::Flex, size: i32) {
        flex.fixed(&mut self.inner.borrow_mut().table, size);
    }

    pub fn activate(&mut self) {
        self.inner.borrow_mut().table.activate();
    }

    pub fn deactivate(&mut self) {
        self.inner.borrow_mut().finish_editing();
        self.inner.borrow_mut().table.deactivate();
    }
}

impl Parametrized for ParamTableWidget {
    fn copy_params_from(&mut self, other: &ParamList) {
        let inner_clone = self.inner.clone();
        let mut inner = inner_clone.borrow_mut();

        inner.params.copy_from(other);
        let len = inner.params.len();
        inner.table.set_rows(len as i32);
        inner.table.redraw();
    }

    fn get_params(&self) -> ParamList {
        self.inner.borrow().params.clone()
    }
}

// Drawing primitives
fn draw_header(txt: &str, x: i32, y: i32, w: i32, h: i32) {
    draw::push_clip(x, y, w, h);
    draw::draw_box(
        enums::FrameType::ThinUpBox,
        x,
        y,
        w,
        h,
        enums::Color::FrameDefault,
    );
    draw::set_draw_color(enums::Color::Black);
    draw::set_font(enums::Font::Helvetica, 14);
    draw::draw_text2(txt, x, y, w, h, enums::Align::Center);
    draw::pop_clip();
}

enum CellState {
    Normal,
    Selected,
    Deactivated,
}

// The selected flag sets the color of the cell to a grayish color, otherwise white
fn draw_data(txt: &str, x: i32, y: i32, w: i32, h: i32, state: CellState) {
    draw::push_clip(x, y, w, h);
    match state {
        CellState::Normal => draw::set_draw_color(enums::Color::White),
        CellState::Selected => draw::set_draw_color(enums::Color::from_u32(0x00F0_F0F0)),
        CellState::Deactivated => draw::set_draw_color(enums::Color::from_u32(0x00D3_D3D3)),
    }
    draw::draw_rectf(x, y, w, h);
    draw::set_draw_color(enums::Color::Gray0);
    draw::set_font(enums::Font::Helvetica, 14);
    draw::draw_text2(txt, x, y, w, h, enums::Align::Center);
    draw::draw_rect(x, y, w, h);
    draw::pop_clip();
}
