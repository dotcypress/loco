#![no_std]
#![no_main]
#![deny(warnings)]

extern crate panic_halt;

use loco::*;

#[cortex_m_rt::entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let mut rcc = dp.RCC.constrain();
    let mut delay = dp.TIM16.delay(&mut rcc);

    let pins = Pins::new(dp.GPIOA, dp.GPIOB, dp.GPIOC, &mut rcc);
    let mut led = led(pins.g8);

    let mut kvs = store::store(
        dp.SPI1,
        pins.flash_sck,
        pins.flash_miso,
        pins.flash_mosi,
        4.MHz(),
        &mut rcc,
    )
    .expect("cannot open flash store");

    kvs.insert(b"key", b"value").ok();

    loop {
        led.toggle().ok();
        delay.delay(500.millis());
    }
}
