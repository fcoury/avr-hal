#![no_std]
#![no_main]

use keyboard_hal::{Keyboard, Pins, UsbBus};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = Pins::new(dp.PORTB, dp.PORTC);
    let usb_bus = UsbBus::new(dp.USB_DEVICE);

    let mut keyboard = Keyboard::new(pins, usb_bus);

    loop {
        keyboard.poll();
    }
}
