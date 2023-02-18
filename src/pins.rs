use crate::hal::gpio::{gpioa::*, gpiob::*, gpioc::*, gpiod::*, gpiof::*};
use hal::gpio::{DefaultMode, Floating, Input, OpenDrain, Output, PushPull};
use hal::prelude::*;
use hal::rcc::Rcc;
use hal::stm32::*;

// SWD
pub type SwdIo = PA13<DefaultMode>;
pub type SwdClk = PA14<DefaultMode>;

/// I2C
pub type I2cScl = PB8<Output<OpenDrain>>;
pub type I2cSda = PB9<Output<OpenDrain>>;

/// UART
pub type UartTx = PB6<DefaultMode>;
pub type UartRx = PB7<DefaultMode>;

/// GPIO
pub type G1 = PA0<DefaultMode>;
pub type G2 = PA1<DefaultMode>;
pub type G3 = PA2<DefaultMode>;
pub type G4 = PA3<DefaultMode>;
pub type G5 = PA4<DefaultMode>;
pub type G6 = PA5<DefaultMode>;

// DC Motors
pub type MotorRef = PB4<DefaultMode>;
pub type MotorFault = PD2<Input<Floating>>;
pub type MotorStandby = PD0<Output<PushPull>>;
pub type MotorEnable = PD1<Output<PushPull>>;
pub type MotorAPhase = PA15<Output<PushPull>>;
pub type MotorAPWM = PA11<DefaultMode>;
pub type MotorBPhase = PA12<Output<PushPull>>;
pub type MotorBPWM = PA10<DefaultMode>;

// Stepper Motors
pub type MotorXRef = PC7<DefaultMode>;
pub type MotorXFault = PC13<Input<Floating>>;
pub type MotorXStandby = PB5<Output<PushPull>>;
pub type MotorXEnable = PC14<Output<PushPull>>;
pub type MotorXStep = PD3<Output<PushPull>>;
pub type MotorXDir = PB3<Output<PushPull>>;

pub type MotorYRef = PB1<DefaultMode>;
pub type MotorYFault = PB10<Input<Floating>>;
pub type MotorYStandby = PB0<Output<PushPull>>;
pub type MotorYEnable = PB2<Output<PushPull>>;
pub type MotorYStep = PA6<Output<PushPull>>;
pub type MotorYDir = PA7<Output<PushPull>>;

pub type MotorZRef = PA8<DefaultMode>;
pub type MotorZFault = PC6<Input<Floating>>;
pub type MotorZStandby = PB15<Output<PushPull>>;
pub type MotorZEnable = PA9<Output<PushPull>>;
pub type MotorZStep = PB14<Output<PushPull>>;
pub type MotorZDir = PB13<Output<PushPull>>;

// Power switches
pub type Switch1Enable = PB12<Output<PushPull>>;
pub type Switch1Sense = PB11<Input<Floating>>;

pub type Switch2Enable = PF1<Output<PushPull>>;
pub type Switch2Sense = PF0<Input<Floating>>;

pub type Switch3Enable = PF3<Output<PushPull>>;
pub type Switch3Sense = PC15<Input<Floating>>;

pub struct Gpio {
    pub g1: G1,
    pub g2: G2,
    pub g3: G3,
    pub g4: G4,
    pub g5: G5,
    pub g6: G6,
}

pub struct Pins {
    // I2C
    pub i2c_scl: I2cScl,
    pub i2c_sda: I2cSda,

    // SWD
    pub swd_io: SwdIo,
    pub swd_clk: SwdClk,

    // UART
    pub uart_tx: UartTx,
    pub uart_rx: UartRx,

    // GPIO
    pub gpio: Gpio,

    // DC Motors
    pub motor_ref: MotorRef,
    pub motor_fault: MotorFault,
    pub motor_standby: MotorStandby,
    pub motor_enable: MotorEnable,
    pub motor_a_phase: MotorAPhase,
    pub motor_a_pwm: MotorAPWM,
    pub motor_b_phase: MotorBPhase,
    pub motor_b_pwm: MotorBPWM,

    // Stepper Motors
    pub motor_x_ref: MotorXRef,
    pub motor_x_fault: MotorXFault,
    pub motor_x_standby: MotorXStandby,
    pub motor_x_enable: MotorXEnable,
    pub motor_x_step: MotorXStep,
    pub motor_x_dir: MotorXDir,

    pub motor_y_ref: MotorYRef,
    pub motor_y_fault: MotorYFault,
    pub motor_y_standby: MotorYStandby,
    pub motor_y_enable: MotorYEnable,
    pub motor_y_step: MotorYStep,
    pub motor_y_dir: MotorYDir,

    pub motor_z_ref: MotorZRef,
    pub motor_z_fault: MotorZFault,
    pub motor_z_standby: MotorZStandby,
    pub motor_z_enable: MotorZEnable,
    pub motor_z_step: MotorZStep,
    pub motor_z_dir: MotorZDir,

    // Power switches
    pub switch1_enable: Switch1Enable,
    pub switch1_sense: Switch1Sense,

    pub switch2_enable: Switch2Enable,
    pub switch2_sense: Switch2Sense,

    pub switch3_enable: Switch3Enable,
    pub switch3_sense: Switch3Sense,
}

impl Pins {
    pub fn new(
        gpioa: GPIOA,
        gpiob: GPIOB,
        gpioc: GPIOC,
        gpiod: GPIOD,
        gpiof: GPIOF,
        rcc: &mut Rcc,
    ) -> Self {
        let port_a = gpioa.split(rcc);
        let port_b = gpiob.split(rcc);
        let port_c = gpioc.split(rcc);
        let port_d = gpiod.split(rcc);
        let port_f = gpiof.split(rcc);

        Self {
            // I2C
            i2c_scl: port_b.pb8.into_open_drain_output_in_state(PinState::High),
            i2c_sda: port_b.pb9.into_open_drain_output_in_state(PinState::High),

            // SWD
            swd_io: port_a.pa13,
            swd_clk: port_a.pa14,

            // UART
            uart_tx: port_b.pb6,
            uart_rx: port_b.pb7,

            // GPIO
            gpio: Gpio {
                g1: port_a.pa0,
                g2: port_a.pa1,
                g3: port_a.pa2,
                g4: port_a.pa3,
                g5: port_a.pa4,
                g6: port_a.pa5,
            },

            // DC Motors
            motor_ref: port_b.pb4,
            motor_fault: port_d.pd2.into(),
            motor_standby: port_d.pd0.into(),
            motor_enable: port_d.pd1.into(),
            motor_a_phase: port_a.pa15.into(),
            motor_b_phase: port_a.pa12.into(),
            motor_a_pwm: port_a.pa11,
            motor_b_pwm: port_a.pa10,

            // Stepper Motors
            motor_x_ref: port_c.pc7,
            motor_x_fault: port_c.pc13.into(),
            motor_x_standby: port_b.pb5.into(),
            motor_x_enable: port_c.pc14.into(),
            motor_x_step: port_d.pd3.into(),
            motor_x_dir: port_b.pb3.into(),

            motor_y_ref: port_b.pb1,
            motor_y_fault: port_b.pb10.into(),
            motor_y_standby: port_b.pb0.into(),
            motor_y_enable: port_b.pb2.into(),
            motor_y_step: port_a.pa6.into(),
            motor_y_dir: port_a.pa7.into(),

            motor_z_ref: port_a.pa8,
            motor_z_fault: port_c.pc6.into(),
            motor_z_standby: port_b.pb15.into(),
            motor_z_enable: port_a.pa9.into(),
            motor_z_step: port_b.pb14.into(),
            motor_z_dir: port_b.pb13.into(),

            // Power switches
            switch1_enable: port_b.pb12.into(),
            switch1_sense: port_b.pb11.into(),

            switch2_enable: port_f.pf1.into(),
            switch2_sense: port_f.pf0.into(),

            switch3_enable: port_f.pf3.into(),
            switch3_sense: port_c.pc15.into(),
        }
    }
}
