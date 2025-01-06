# keyboard-hal

Building:

```shell
cd examples/keyboard-dz60
cargo build --release
cd -
avr-objcopy -O ihex -R .eeprom target/avr-atmega32u4/release/simple.elf dz60.hex
```

Connect the keyboard and press reset. Then on macOS:

```shell
dfu-programmer erase --force
dfu-programmer atmega32u4 flash dz60.hex
dfu-programmer atmega32u4 reset
```
