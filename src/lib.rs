#![no_std]

pub extern crate stm32c0xx_hal as hal;

pub mod motor;
pub mod pins;
pub mod power;
pub mod stepper;

use hal::stm32;
use hal::timer::pwm::PwmPin;
use hal::*;
use pins::*;

pub use hal::prelude::*;

pub type MotorTimer = stm32::TIM1;
pub type RefTimer = stm32::TIM3;
pub type I2cDev = i2c::I2c<stm32::I2C, I2cSda, I2cScl>;
pub type UartDev = serial::Serial<stm32::USART1>;

pub type PowerSwitch1 = power::PowerSwitch<Switch1Enable, Switch1Sense>;
pub type PowerSwitch2 = power::PowerSwitch<Switch2Enable, Switch2Sense>;
pub type PowerSwitch3 = power::PowerSwitch<Switch3Enable, Switch3Sense>;

pub type StepperMotor1 = stepper::StepperMotor<
    Stepper1Step,
    Stepper1Dir,
    Stepper1Enable,
    Stepper1Standby,
    Stepper1Fault,
    PwmPin<RefTimer, timer::Channel2>,
    32,
>;

pub type StepperMotor2 = stepper::StepperMotor<
    Stepper2Step,
    Stepper2Dir,
    Stepper2Enable,
    Stepper2Standby,
    Stepper2Fault,
    PwmPin<RefTimer, timer::Channel4>,
    32,
>;

pub type StepperMotor3 = stepper::StepperMotor<
    Stepper3Step,
    Stepper3Dir,
    Stepper3Enable,
    Stepper3Standby,
    Stepper3Fault,
    PwmPin<RefTimer, timer::Channel3>,
    32,
>;

#[derive(Clone, Copy, Debug)]
pub enum Status {
    Busy,
    Idle,
    Fault,
}
