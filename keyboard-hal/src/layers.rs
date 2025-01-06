use crate::{
    keyboard_config::{MATRIX_COLS, MATRIX_ROWS, NUM_LAYERS},
    keycodes::Keycode,
};

pub struct Layers {
    keymaps: [[[Keycode; MATRIX_COLS]; MATRIX_ROWS]; NUM_LAYERS],
    pub current_layer: usize,
}

impl Layers {
    pub fn new() -> Self {
        use Keycode::*;

        // Create the layout matrices
        let base_layer = [
            [
                GraveEsc, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9, Num0, Minus, Equal,
                BSpace, No,
            ],
            [
                Tab, Q, W, E, R, T, Y, U, I, O, P, LBracket, RBracket, BSlash, No,
            ],
            [
                Caps, A, S, D, F, G, H, J, K, L, Semicolon, Quote, Enter, No, No,
            ],
            [
                LShift,
                Z,
                X,
                C,
                V,
                B,
                N,
                M,
                Comma,
                Dot,
                Slash,
                RShift,
                MomentaryLayer1,
                No,
                No,
            ],
            [
                LCtrl, LGui, LAlt, Space, No, No, No, No, No, No, RAlt, RGui, RCtrl, No, No,
            ],
        ];

        let fn_layer = [
            [
                Grave, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, Delete, No,
            ],
            [
                Trans, Trans, Up, Trans, Trans, Trans, Trans, Trans, Trans, Trans, PScreen,
                ScrollLock, Pause, Reset, No,
            ],
            [
                Trans, Left, Down, Right, Trans, Trans, Trans, Trans, Trans, Insert, Home, PgUp,
                Trans, No, No,
            ],
            [
                Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, End, PgDown, Trans,
                Trans, No, No,
            ],
            [
                Trans,
                Trans,
                Trans,
                Trans,
                No,
                No,
                No,
                No,
                No,
                No,
                Trans,
                MomentaryLayer2,
                Trans,
                No,
                No,
            ],
        ];

        let ctrl_layer = [
            [
                Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans,
                Trans, Trans, No,
            ],
            [
                Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans,
                Trans, Trans, No,
            ],
            [
                Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans,
                Trans, No, No,
            ],
            [
                Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans,
                Trans, No, No,
            ],
            [
                Trans, Trans, Trans, Trans, No, No, No, No, No, No, Trans, Trans, Trans, No, No,
            ],
        ];

        let keymaps = [base_layer, fn_layer, ctrl_layer];

        Layers {
            keymaps,
            current_layer: 0,
        }
    }

    pub fn get_keycode(&self, layer: usize, row: usize, col: usize) -> Keycode {
        let keycode = self.keymaps[layer][row][col];
        if keycode == Keycode::Trans && layer > 0 {
            // If the key is transparent, get the keycode from the layer below
            self.get_keycode(layer - 1, row, col)
        } else {
            keycode
        }
    }

    pub fn handle_momentary_layer(&mut self, keycode: Keycode, pressed: bool) {
        match keycode {
            Keycode::MomentaryLayer1 => {
                if pressed {
                    self.current_layer = 1;
                } else {
                    self.current_layer = 0;
                }
            }
            Keycode::MomentaryLayer2 => {
                if pressed {
                    self.current_layer = 2;
                } else {
                    self.current_layer = 0;
                }
            }
            _ => {}
        }
    }
}
