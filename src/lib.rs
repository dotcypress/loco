#![no_std]

pub extern crate stm32c0xx_hal as hal;

pub mod motor;
pub mod pins;
pub mod power;
pub mod stepper;

use hal::rcc::Rcc;
use hal::stm32;
use hal::timer::pwm::{Pwm, PwmPin};
use hal::*;
use motor::MotorController;
use pins::*;

pub use hal::prelude::*;
use power::PowerSwitch;

pub type MotorTimer = stm32::TIM1;
pub type RefTimer = stm32::TIM3;
pub type I2cDev = i2c::I2c<stm32::I2C, I2cSda, I2cScl>;
pub type UartDev = serial::Serial<stm32::USART1>;

pub type PowerSwitch1 = power::PowerSwitch<Switch1Enable, Switch1Sense>;
pub type PowerSwitch2 = power::PowerSwitch<Switch2Enable, Switch2Sense>;
pub type PowerSwitch3 = power::PowerSwitch<Switch3Enable, Switch3Sense>;

pub type MotorX = stepper::StepperMotor<
    MotorXStep,
    MotorXDir,
    MotorXEnable,
    MotorXStandby,
    MotorXFault,
    PwmPin<RefTimer, timer::Channel2>,
    32,
>;

pub type MotorY = stepper::StepperMotor<
    MotorYStep,
    MotorYDir,
    MotorYEnable,
    MotorYStandby,
    MotorYFault,
    PwmPin<RefTimer, timer::Channel4>,
    32,
>;

pub type MotorZ = stepper::StepperMotor<
    MotorZStep,
    MotorZDir,
    MotorZEnable,
    MotorZStandby,
    MotorZFault,
    PwmPin<RefTimer, timer::Channel3>,
    32,
>;

#[derive(Clone, Copy, Debug)]
pub enum Status {
    Busy,
    Idle,
    Fault,
}

pub struct Loco {
    pub motor_ab: MotorController,
    pub motor_x: MotorX,
    pub motor_y: MotorY,
    pub motor_z: MotorZ,
    pub power_switch1: PowerSwitch1,
    pub power_switch2: PowerSwitch2,
    pub power_switch3: PowerSwitch3,
    pub serial: UartDev,
    pub i2c: I2cDev,
    pub gpio: Gpio,
}

impl Loco {
    pub fn new(
        pins: Pins,
        usart: stm32::USART1,
        i2c: stm32::I2C,
        ref_pwm: &mut Pwm<RefTimer>,
        motor_pwm: &mut Pwm<MotorTimer>,
        rcc: &mut Rcc,
    ) -> Self {
        let serial = usart
            .usart((pins.uart_tx, pins.uart_rx), Default::default(), rcc)
            .unwrap();
        let i2c = i2c.i2c(pins.i2c_sda, pins.i2c_scl, 400.kHz(), rcc);

        let motor_ab = MotorController::new(
            ref_pwm.bind_pin(pins.motor_ref),
            (
                motor_pwm.bind_pin(pins.motor_a_pwm),
                motor_pwm.bind_pin(pins.motor_b_pwm),
            ),
            (pins.motor_a_phase, pins.motor_b_phase),
            pins.motor_enable,
            pins.motor_standby,
            pins.motor_fault,
        );

        let motor_x = MotorX::new(
            pins.motor_x_step,
            pins.motor_x_dir,
            pins.motor_x_enable,
            pins.motor_x_standby,
            pins.motor_x_fault,
            ref_pwm.bind_pin(pins.motor_x_ref),
        );

        let motor_y = MotorY::new(
            pins.motor_y_step,
            pins.motor_y_dir,
            pins.motor_y_enable,
            pins.motor_y_standby,
            pins.motor_y_fault,
            ref_pwm.bind_pin(pins.motor_y_ref),
        );

        let motor_z = MotorZ::new(
            pins.motor_z_step,
            pins.motor_z_dir,
            pins.motor_z_enable,
            pins.motor_z_standby,
            pins.motor_z_fault,
            ref_pwm.bind_pin(pins.motor_z_ref),
        );

        let power_switch1 = PowerSwitch::new(
            pins.switch1_enable,
            pins.switch1_sense,
        );

        let power_switch2 = PowerSwitch::new(
            pins.switch2_enable,
            pins.switch2_sense,
        );

        let power_switch3 = PowerSwitch::new(
            pins.switch3_enable,
            pins.switch3_sense,
        );

        let gpio = pins.gpio;

        Self {
            motor_ab,
            motor_x,
            motor_y,
            motor_z,
            power_switch1,
            power_switch2,
            power_switch3,
            serial,
            i2c,
            gpio,
        }
    }
}
