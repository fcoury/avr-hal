use avr_device::atmega32u4::USB_DEVICE as USB;
use core::sync::atomic::{AtomicBool, Ordering};
use usb_device::{
    bus::PollResult,
    endpoint::{EndpointAddress, EndpointType},
    UsbDirection,
};
use usbd_hid::UsbError;

const MAX_ENDPOINTS: usize = 7;

#[derive(Clone, Copy)]
#[allow(unused)]
struct Endpoint {
    ep_type: EndpointType,
    max_size: u16,
    buffer_offset: usize,
}

pub struct UsbBus {
    usb: USB,
    dpram: [u8; 1024],
    endpoints: [Option<Endpoint>; MAX_ENDPOINTS],
    next_buffer_offset: usize,
    configured: AtomicBool,
}

unsafe impl Sync for UsbBus {}

impl UsbBus {
    pub fn new(usb: USB) -> Self {
        // Enable USB peripheral and PLL
        let peripherals = avr_device::atmega32u4::Peripherals::take().unwrap();

        // Start PLL
        peripherals.PLL.pllcsr.write(|w| w.plle().set_bit());

        // Wait for PLL lock
        while peripherals.PLL.pllcsr.read().plock().bit_is_clear() {}

        // Enable USB peripheral
        usb.usbcon
            .modify(|_, w| w.usbe().set_bit().frzclk().clear_bit());

        let none_endpoint: Option<Endpoint> = None;
        UsbBus {
            usb,
            dpram: [0; 1024],
            endpoints: [none_endpoint; MAX_ENDPOINTS],
            next_buffer_offset: 0,
            configured: AtomicBool::new(false),
        }
    }

    fn get_endpoint_type_bits(ep_type: EndpointType) -> u8 {
        // Convert endpoint type to ATmega32U4 endpoint type bits
        // Control: 0, Isochronous: 1, Bulk: 2, Interrupt: 3
        let val = match ep_type {
            EndpointType::Control => 0,
            EndpointType::Bulk => 2,
            EndpointType::Interrupt => 3,
            _ => 1, // Isochronous
        };
        val
    }

    fn get_endpoint_size_bits(max_packet_size: u16) -> Result<u8, UsbError> {
        match max_packet_size {
            8 => Ok(0),
            16 => Ok(1),
            32 => Ok(2),
            64 => Ok(3),
            _ => Err(UsbError::Unsupported),
        }
    }
}

impl usb_device::bus::UsbBus for UsbBus {
    fn alloc_ep(
        &mut self,
        ep_dir: UsbDirection,
        ep_addr: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet_size: u16,
        _interval: u8,
    ) -> Result<EndpointAddress, UsbError> {
        let idx = match ep_addr {
            Some(addr) => addr.index(),
            None => self
                .endpoints
                .iter()
                .position(|ep| ep.is_none())
                .ok_or(UsbError::EndpointOverflow)?,
        };

        if self.next_buffer_offset + max_packet_size as usize > self.dpram.len() {
            return Err(UsbError::EndpointMemoryOverflow);
        }

        let ep = Endpoint {
            ep_type,
            max_size: max_packet_size,
            buffer_offset: self.next_buffer_offset,
        };

        self.next_buffer_offset += max_packet_size as usize;
        self.endpoints[idx] = Some(ep);

        let ep_num = idx as u8;
        let ep_type_bits = Self::get_endpoint_type_bits(ep_type);
        let ep_size_bits = Self::get_endpoint_size_bits(max_packet_size)?;

        match ep_dir {
            UsbDirection::Out => {
                self.usb.uenum.write(|w| w.bits(ep_num));
                self.usb.ueconx.write(|w| w.epen().set_bit());
                self.usb
                    .uecfg0x
                    .write(|w| w.eptype().bits(ep_type_bits).epdir().clear_bit());
                self.usb.uecfg1x.write(|w| w.epsize().bits(ep_size_bits));
            }
            UsbDirection::In => {
                self.usb.uenum.write(|w| w.bits(ep_num));
                self.usb.ueconx.write(|w| w.epen().set_bit());
                self.usb
                    .uecfg0x
                    .write(|w| w.eptype().bits(ep_type_bits).epdir().set_bit());
                self.usb.uecfg1x.write(|w| w.epsize().bits(ep_size_bits));
            }
        }

        Ok(EndpointAddress::from_parts(idx, ep_dir))
    }

    fn enable(&mut self) {
        self.configured.store(true, Ordering::SeqCst);
        self.usb.udcon.modify(|_, w| w.detach().clear_bit());
    }

    fn reset(&self) {
        self.configured.store(false, Ordering::SeqCst);

        for i in 0..MAX_ENDPOINTS {
            self.usb.uenum.write(|w| w.bits(i as u8));
            self.usb.ueconx.write(|w| w.epen().clear_bit());
        }

        self.usb.udint.write(|w| unsafe { w.bits(0) });
    }

    fn set_device_address(&self, addr: u8) {
        self.usb
            .udaddr
            .write(|w| w.uadd().bits(addr).adden().set_bit());
    }

    fn write(&self, ep_addr: EndpointAddress, buf: &[u8]) -> Result<usize, UsbError> {
        let idx = ep_addr.index();
        if let Some(ep) = &self.endpoints[idx] {
            self.usb.uenum.write(|w| w.bits(idx as u8));

            while self.usb.ueintx.read().txini().bit_is_clear() {}

            let count = buf.len().min(ep.max_size as usize);
            for &byte in buf[..count].iter() {
                self.usb.uedatx.write(|w| w.bits(byte));
            }

            self.usb
                .ueintx
                .modify(|_, w| w.txini().clear_bit().fifocon().clear_bit());

            Ok(count)
        } else {
            Err(UsbError::InvalidEndpoint)
        }
    }

    fn read(&self, ep_addr: EndpointAddress, buf: &mut [u8]) -> Result<usize, UsbError> {
        let idx = ep_addr.index();
        if let Some(ep) = &self.endpoints[idx] {
            self.usb.uenum.write(|w| w.bits(idx as u8));

            while self.usb.ueintx.read().rxouti().bit_is_clear() {}

            let mut count = 0;
            while count < buf.len() && count < ep.max_size as usize {
                buf[count] = self.usb.uedatx.read().bits();
                count += 1;
            }

            self.usb
                .ueintx
                .modify(|_, w| w.rxouti().clear_bit().fifocon().clear_bit());

            Ok(count)
        } else {
            Err(UsbError::InvalidEndpoint)
        }
    }

    fn set_stalled(&self, ep_addr: EndpointAddress, stalled: bool) {
        let idx = ep_addr.index();
        self.usb.uenum.write(|w| w.bits(idx as u8));
        self.usb.ueconx.modify(|_, w| w.stallrq().bit(stalled));
    }

    fn is_stalled(&self, ep_addr: EndpointAddress) -> bool {
        let idx = ep_addr.index();
        self.usb.uenum.write(|w| w.bits(idx as u8));
        self.usb.ueconx.read().stallrq().bit()
    }

    fn suspend(&self) {
        self.usb.udcon.modify(|_, w| w.detach().set_bit());
    }

    fn resume(&self) {
        self.usb.udcon.modify(|_, w| w.detach().clear_bit());
    }

    fn poll(&self) -> PollResult {
        let udint = self.usb.udint.read();
        let mut result = PollResult::None;

        if udint.eorsti().bit() {
            self.usb.udint.modify(|_, w| w.eorsti().clear_bit());
            result = PollResult::Reset;
        }

        if udint.suspi().bit() {
            self.usb.udint.modify(|_, w| w.suspi().clear_bit());
            result = PollResult::Suspend;
        }

        if udint.wakeupi().bit() {
            self.usb.udint.modify(|_, w| w.wakeupi().clear_bit());
            result = PollResult::Resume;
        }

        result
    }
}

#[macro_export]
macro_rules! usb_bus {
    ($dp:expr) => {{
        let usb = $dp.USB_DEVICE;
        static mut USB_BUS: Option<UsbBusAllocator<UsbBus>> = None;

        unsafe {
            USB_BUS = Some(UsbBusAllocator::new(UsbBus::new(usb)));
            USB_BUS.as_ref().unwrap()
        }
    }};
}
