#![no_std]

pub extern crate stm32g0xx_hal as hal;

mod pins;
#[cfg(feature = "store")]
pub mod store;

pub use hal::prelude::*;
pub use hal::stm32;
pub use pins::*;

use hal::time::Hertz;
use hal::{i2c, rcc::Rcc, serial, spi, timer::qei};

pub type SpiDev = spi::Spi<stm32::SPI1, (FlashSck, FlashMiso, FlashMosi)>;
pub type I2cDev = i2c::I2c<stm32::I2C1, I2cSda, I2cScl>;
pub type UartDev = serial::Serial<stm32::USART2, serial::BasicConfig>;
pub type EncoderDev = qei::Qei<stm32::TIM1, (Enc1, Enc2)>;

pub fn led<T: Into<LedPin>>(led_pin: T) -> LedPin {
    led_pin.into()
}

pub fn flash(
    spi_dev: stm32::SPI1,
    scl: FlashSck,
    miso: FlashMiso,
    mosi: FlashMosi,
    speed: Hertz,
    rcc: &mut Rcc,
) -> SpiDev {
    spi_dev.spi((scl, miso, mosi), spi::MODE_0, speed, rcc)
}

pub fn i2c(i2c_dev: stm32::I2C1, sda: I2cSda, scl: I2cScl, speed: Hertz, rcc: &mut Rcc) -> I2cDev {
    i2c_dev.i2c(sda, scl, speed, rcc)
}

pub struct Serial {
    pub dev: UartDev,
}

pub fn serial(
    uart_dev: stm32::USART2,
    tx: UartTx,
    rx: UartRx,
    cfg: serial::BasicConfig,
    rcc: &mut Rcc,
) -> UartDev {
    uart_dev.usart(tx, rx, cfg, rcc).unwrap()
}

pub struct Encoder {
    pub dev: EncoderDev,
}

pub fn encoder(encoder_dev: stm32::TIM1, enc1: Enc1, enc2: Enc2, rcc: &mut Rcc) -> EncoderDev {
    encoder_dev.qei((enc1, enc2), rcc)
}
