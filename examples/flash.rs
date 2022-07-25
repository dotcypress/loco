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

    let mut flash = flash(
        dp.SPI1,
        pins.flash_sck,
        pins.flash_miso,
        pins.flash_mosi,
        4.mhz(),
        &mut rcc,
    );

    loop {
        led.toggle().ok();
        flash.read().unwrap();
        delay.delay(500.ms());
    }
}
