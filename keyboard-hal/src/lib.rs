#![no_std]

pub use usb_device::prelude::*;
use usb_keyboard::UsbKeyboard;
use usbd_hid::{
    descriptor::{KeyboardReport, SerializedDescriptor},
    hid_class::HIDClass,
};

// re-exports
pub use atmega_hal as hal;
pub use atmega_hal::pac;
pub use avr_device::entry;
pub use hal::Peripherals;
pub use usb_device::bus::UsbBusAllocator;
pub use usb_device::device::{
    StringDescriptors, UsbDevice, UsbDeviceBuilder, UsbDeviceState, UsbVidPid,
};
pub use usb_device::LangID;
pub use usb_device::UsbError;

use keyboard_config::{MATRIX_COLS, MATRIX_ROWS};
use layers::Layers;
use matrix::Matrix;
pub use port::pcb1::Pins;
pub use usb::UsbBus;

pub mod keyboard_config;
pub mod keycodes;
pub mod layers;
pub mod matrix;
pub mod port;
pub mod usb;
pub mod usb_keyboard;

pub struct Keyboard<B: usb_device::bus::UsbBus + 'static> {
    matrix: Matrix,
    layers: Layers,
    usb_keyboard: UsbKeyboard<B>,
}

impl<B: usb_device::bus::UsbBus + 'static> Keyboard<B> {
    pub fn new(pins: Pins, usb_bus: &'static UsbBusAllocator<B>) -> Self {
        // Configure row pins as outputs
        let rows = [
            pins.row0.into_output().downgrade(),
            pins.row1.into_output().downgrade(),
            pins.row2.into_output().downgrade(),
            pins.row3.into_output().downgrade(),
            pins.row4.into_output().downgrade(),
        ];

        // Configure column pins as pull-up inputs
        let cols = [
            pins.col0.into_pull_up_input().downgrade().forget_imode(),
            pins.col1.into_pull_up_input().downgrade().forget_imode(),
            pins.col2.into_pull_up_input().downgrade().forget_imode(),
            pins.col3.into_pull_up_input().downgrade().forget_imode(),
            pins.col4.into_pull_up_input().downgrade().forget_imode(),
            pins.col5.into_pull_up_input().downgrade().forget_imode(),
            pins.col6.into_pull_up_input().downgrade().forget_imode(),
            pins.col7.into_pull_up_input().downgrade().forget_imode(),
            pins.col8.into_pull_up_input().downgrade().forget_imode(),
            pins.col9.into_pull_up_input().downgrade().forget_imode(),
            pins.col10.into_pull_up_input().downgrade().forget_imode(),
            pins.col11.into_pull_up_input().downgrade().forget_imode(),
            pins.col12.into_pull_up_input().downgrade().forget_imode(),
            pins.col13.into_pull_up_input().downgrade().forget_imode(),
            pins.col14.into_pull_up_input().downgrade().forget_imode(),
        ];

        // Initialize the matrix with the configured pins
        let matrix = Matrix::new(rows, cols);
        let layers = Layers::new();

        let hid_class = HIDClass::new(&usb_bus, KeyboardReport::desc(), 1);
        let usb_device = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x445A, 0x2260)).build();

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

                    // Handle layer switching first
                    self.layers
                        .handle_momentary_layer(keycode, new_state[row][col]);

                    // Only send regular keycodes to USB
                    if keycode as u8 <= 0xE7 {
                        // Regular HID keycodes only
                        self.usb_keyboard
                            .handle_keypress(keycode as u8, new_state[row][col]);
                    }
                }
            }
        }
        self.matrix.last_state = new_state;
    }
}

/// Convenience macro to instantiate the [`Pins`] struct for this board.
///
/// # Example
/// ```no_run
/// let dp = arduino_hal::Peripherals::take().unwrap();
/// let pins = arduino_hal::pins!(dp);
/// ```
#[macro_export]
macro_rules! pins {
    ($p:expr) => {
        $crate::Pins::with_mcu_pins($crate::hal::pins!($p))
    };
}
