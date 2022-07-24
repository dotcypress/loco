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
    let port_a = dp.GPIOA.split(&mut rcc);
    let mut led = port_a.pa5.into_open_drain_output();

    loop {
        led.toggle().ok();
        delay.delay(200.ms());
    }
}
