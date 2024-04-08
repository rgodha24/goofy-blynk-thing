deploy:
  cargo build --release --target thumbv6m-none-eabi

  rust-objcopy -O binary target/thumbv6m-none-eabi/release/goofy-blynk-thing target/goofy-blynk-thing.bin

  arduino-cli upload -i target/goofy-blynk-thing.bin -b arduino:samd:nano_33_iot -p /dev/ttyACM0
