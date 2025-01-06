use atmega_hal::port::Pin;

avr_hal_generic::renamed_pins! {
    pub struct Pins {
        // Row pins (D0-D3, D5)
        pub row0: atmega_hal::port::PD0 = pd0,
        pub row1: atmega_hal::port::PD1 = pd1,
        pub row2: atmega_hal::port::PD2 = pd2,
        pub row3: atmega_hal::port::PD3 = pd3,
        pub row4: atmega_hal::port::PD5 = pd5,

        // Column pins (F0, F1, E6, C7, C6, B7, D4, B1, B0, B5, B4, D7, D6, B3, F4)
        pub col0: atmega_hal::port::PF0 = pf0,
        pub col1: atmega_hal::port::PF1 = pf1,
        pub col2: atmega_hal::port::PE6 = pe6,
        pub col3: atmega_hal::port::PC7 = pc7,
        pub col4: atmega_hal::port::PC6 = pc6,
        pub col5: atmega_hal::port::PB7 = pb7,
        pub col6: atmega_hal::port::PD4 = pd4,
        pub col7: atmega_hal::port::PB1 = pb1,
        pub col8: atmega_hal::port::PB0 = pb0,
        pub col9: atmega_hal::port::PB5 = pb5,
        pub col10: atmega_hal::port::PB4 = pb4,
        pub col11: atmega_hal::port::PD7 = pd7,
        pub col12: atmega_hal::port::PD6 = pd6,
        pub col13: atmega_hal::port::PB3 = pb3,
        pub col14: atmega_hal::port::PF4 = pf4,
    }

    impl Pins {
        type Pin = Pin;
        type McuPins = atmega_hal::Pins;
    }
}
