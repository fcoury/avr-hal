use crate::{keyboard_config::{MATRIX_COLS, MATRIX_ROWS, NUM_LAYERS}, keycodes::Keycode};

pub struct Layers {
    keymaps: [[[Keycode; MATRIX_COLS]; MATRIX_ROWS]; NUM_LAYERS],
    pub current_layer: usize,
}

impl Layers {
    pub fn new() -> Self {
        // Initialize with default keymaps
        Layers {
            keymaps: [[[Keycode::No; MATRIX_COLS]; MATRIX_ROWS]; NUM_LAYERS],
            current_layer: 0,
        }
    }

    pub fn get_keycode(&self, layer: usize, row: usize, col: usize) -> Keycode {
        self.keymaps[layer][row][col]
    }
}
