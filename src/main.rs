#![no_std]
#![no_main]

use {defmt_rtt as _, panic_probe as _};

use arduino_nano33iot as bsp;
use bsp::hal;


use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::time::MegaHertz;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut led: bsp::Led = pins.led_sck.into();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    // let mut blynk = blynk_io::Blynk::new("i4yPswMJdpwrkQvdCO3brClifCeNa0kn");


    loop {
        delay.delay_ms(255u8);
        led.set_high().unwrap();
        delay.delay_ms(255u8);
        led.set_low().unwrap();

        serial.write(b'h').unwrap();
    }
}
