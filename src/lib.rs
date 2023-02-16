#![no_std]

pub extern crate stm32c0xx_hal as hal;

use hal::timer::pwm::PwmPin;
use hal::*;
use pins::*;

pub use hal::prelude::*;
pub use hal::stm32;

pub mod pins;
pub mod stepper;

pub type I2cDev = i2c::I2c<stm32::I2C, I2cSda, I2cScl>;
pub type UartDev = serial::Serial<stm32::USART1>;
pub type MotorsRefTimer = stm32::TIM1;
pub type SteppersRefTimer = stm32::TIM3;

pub type StepperMotor1 = stepper::StepperMotor<
    Stepper1Step,
    Stepper1Dir,
    Stepper1Enable,
    Stepper1Standby,
    PwmPin<SteppersRefTimer, Stepper1Ref>,
    32
>;

pub type StepperMotor2 = stepper::StepperMotor<
    Stepper2Step,
    Stepper2Dir,
    Stepper2Enable,
    Stepper2Standby,
    PwmPin<SteppersRefTimer, Stepper2Ref>,
    32
>;

pub type StepperMotor3 = stepper::StepperMotor<
    Stepper3Step,
    Stepper3Dir,
    Stepper3Enable,
    Stepper3Standby,
    PwmPin<SteppersRefTimer, Stepper3Ref>,
    32
>;
