use atmega_hal::port::Pin;

avr_hal_generic::renamed_pins! {
    pub struct Pins {
        /// Row 0
        pub row0: atmega_hal::port::PB0 = pb0,
        /// Row 1
        pub row1: atmega_hal::port::PB1 = pb1,
        /// Column 0
        pub col0: atmega_hal::port::PB2 = pb2,
        /// Column 1
        pub col1: atmega_hal::port::PB3 = pb3,
        // ...
    }

    impl Pins {
        type Pin = Pin;
        type McuPins = atmega_hal::Pins;
    }
}
