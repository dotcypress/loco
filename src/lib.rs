#![no_std]

pub extern crate stm32c0xx_hal as hal;

use hal::time::Hertz;
use hal::{i2c, rcc::Rcc, serial};
use pins::*;

pub use hal::prelude::*;
pub use hal::stm32;

pub mod pins;
pub mod stepper;

pub type I2cDev = i2c::I2c<stm32::I2C, I2cSda, I2cScl>;
pub type UartDev = serial::Serial<stm32::USART1>;

pub fn i2c(i2c_dev: stm32::I2C, sda: I2cSda, scl: I2cScl, speed: Hertz, rcc: &mut Rcc) -> I2cDev {
    i2c_dev.i2c(sda, scl, speed, rcc)
}

pub fn serial(
    uart_dev: stm32::USART1,
    tx: UartTx,
    rx: UartRx,
    cfg: serial::Config,
    rcc: &mut Rcc,
) -> UartDev {
    uart_dev.usart((tx, rx), cfg, rcc).unwrap()
}
