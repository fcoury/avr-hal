use usb_device::{bus::UsbBus, device::UsbDevice};
use usbd_hid::{descriptor::KeyboardReport, hid_class::HIDClass};

pub struct UsbKeyboard<B: UsbBus + 'static> {
    usb_device: UsbDevice<'static, B>,
    hid_class: HIDClass<'static, B>,
    last_report: KeyboardReport,
}

impl<B: UsbBus> UsbKeyboard<B> {
    pub fn new(usb_device: UsbDevice<'static, B>, hid_class: HIDClass<'static, B>) -> Self {
        UsbKeyboard {
            usb_device,
            hid_class,
            last_report: KeyboardReport::default(),
        }
    }

    pub fn handle_keypress(&mut self, keycode: u8, pressed: bool) {
        if pressed {
            // Add keycode to the report
            if let Some(slot) = self
                .last_report
                .keycodes
                .iter_mut()
                .find(|slot| **slot == 0)
            {
                *slot = keycode;
            }
        } else {
            // Remove keycode from the report
            for slot in self.last_report.keycodes.iter_mut() {
                if *slot == keycode {
                    *slot = 0;
                }
            }
        }

        self.send_report();
    }

    fn send_report(&mut self) {
        if self.usb_device.poll(&mut [&mut self.hid_class]) {
            self.hid_class.push_input(&self.last_report).ok();
        }
    }
}
