use crate::keyboard_config::{MATRIX_COLS, MATRIX_ROWS};
use atmega_hal::port::{
    mode::{AnyInput, Input, Output},
    Pin,
};

pub struct Matrix {
    pub last_state: [[bool; MATRIX_COLS]; MATRIX_ROWS],
    rows: [Pin<Output>; MATRIX_ROWS],
    cols: [Pin<Input<AnyInput>>; MATRIX_COLS],
}

impl Matrix {
    pub fn new(
        rows: [Pin<Output>; MATRIX_ROWS],
        cols: [Pin<Input<AnyInput>>; MATRIX_COLS],
    ) -> Self {
        Matrix {
            last_state: [[false; MATRIX_COLS]; MATRIX_ROWS],
            rows,
            cols,
        }
    }

    pub fn scan(&mut self) -> [[bool; MATRIX_COLS]; MATRIX_ROWS] {
        let mut new_state = [[false; MATRIX_COLS]; MATRIX_ROWS];

        for (row_idx, row_pin) in self.rows.iter_mut().enumerate() {
            row_pin.set_low();

            for (col_idx, col_pin) in self.cols.iter().enumerate() {
                new_state[row_idx][col_idx] = col_pin.is_low();
            }

            row_pin.set_high();
        }

        new_state
    }
}
