#![no_std]
#![no_main]

use keyboard_hal::{
    pac, pins, usb, usb_bus, Keyboard, LangID, Pins, StringDescriptors, UsbBus, UsbBusAllocator,
    UsbDeviceBuilder, UsbVidPid,
};
use panic_halt as _;

#[keyboard_hal::entry]
fn main() -> ! {
    let dp = keyboard_hal::Peripherals::take().unwrap();
    let pins = pins!(dp);

    // Get the USB bus via our macro
    let usb_bus = usb_bus!(dp);

    // Create our keyboard instance
    let mut keyboard = Keyboard::new(pins, usb_bus);

    loop {
        keyboard.poll();
    }
}
