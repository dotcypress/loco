#![no_std]
#![no_main]
#![deny(warnings)]

extern crate panic_halt;
use core::fmt::Write;
use hal::serial::BasicConfig;
use loco::*;

#[cortex_m_rt::entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let mut rcc = dp.RCC.constrain();
    let mut delay = dp.TIM16.delay(&mut rcc);

    let pins = Pins::new(dp.GPIOA, dp.GPIOB, dp.GPIOC, &mut rcc);
    let mut led = led(pins.g8);

    let uart_cfg = BasicConfig::default().baudrate(115_200.bps());
    let mut serial = serial(dp.USART2, pins.uart_tx, pins.uart_rx, uart_cfg, &mut rcc);

    loop {
        led.toggle().ok();
        write!(serial, "tick\r\n").ok();
        delay.delay(500.ms());
    }
}
