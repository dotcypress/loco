#![no_std]
#![no_main]
#![deny(warnings)]

extern crate panic_halt;

use hal::rcc::Config;
use loco::*;

#[cortex_m_rt::entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let mut rcc = dp.RCC.freeze(Config::pll());
    let mut delay = dp.TIM16.delay(&mut rcc);

    let pins = Pins::new(dp.GPIOA, dp.GPIOB, dp.GPIOC, &mut rcc);
    let mut led = led(pins.g8);
    led.set_high().ok();

    let mut kvs = store(
        dp.SPI1,
        pins.flash_sck,
        pins.flash_miso,
        pins.flash_mosi,
        pins.swd_io.into(),
        4.MHz(),
        &mut rcc,
    )
    .expect("cannot open flash store");

    kvs.insert(b"key", b"value").unwrap();

    loop {
        if kvs.lookup(b"key").is_ok() {
            led.toggle().ok();
        }
        delay.delay(100.millis());
    }
}
