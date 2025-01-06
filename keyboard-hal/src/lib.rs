#![no_std]

pub use usb_device::prelude::*;
use usb_device::{
    bus::{UsbBus, UsbBusAllocator},
    device::{UsbDeviceBuilder, UsbVidPid},
};
use usb_keyboard::UsbKeyboard;
use usbd_hid::{
    descriptor::{KeyboardReport, SerializedDescriptor},
    hid_class::HIDClass,
};

use keyboard_config::{MATRIX_COLS, MATRIX_ROWS};
use layers::Layers;
use matrix::Matrix;
pub use port::pcb1::Pins;

pub mod keyboard_config;
pub mod keycodes;
pub mod layers;
pub mod matrix;
pub mod port;
pub mod usb_keyboard;

pub struct Keyboard<B: UsbBus + 'static> {
    matrix: Matrix,
    layers: Layers,
    usb_keyboard: UsbKeyboard<B>,
}

impl<B: UsbBus + 'static> Keyboard<B> {
    pub fn new(pins: Pins, usb_bus: &'static UsbBusAllocator<B>) -> Self {
        let row0 = pins.row0.into_output().downgrade();
        let row1 = pins.row1.into_output().downgrade();

        let col0 = pins.col0.into_pull_up_input().downgrade().forget_imode();
        let col1 = pins.col1.into_pull_up_input().downgrade().forget_imode();

        let matrix = Matrix::new([row0, row1], [col0, col1]);
        let layers = Layers::new();

        let hid_class = HIDClass::new(&usb_bus, KeyboardReport::desc(), 1);
        let usb_device = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16C0, 0x27DB)).build();

        Keyboard {
            matrix,
            layers,
            usb_keyboard: UsbKeyboard::new(usb_device, hid_class),
        }
    }

    pub fn poll(&mut self) {
        let new_state = self.matrix.scan();
        for row in 0..MATRIX_ROWS {
            for col in 0..MATRIX_COLS {
                if new_state[row][col] != self.matrix.last_state[row][col] {
                    let keycode = self.layers.get_keycode(self.layers.current_layer, row, col);
                    self.usb_keyboard
                        .handle_keypress(keycode as u8, new_state[row][col]);
                }
            }
        }
        self.matrix.last_state = new_state;
    }
}
