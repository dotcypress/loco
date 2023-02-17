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
pub type Motor1Phase = PA12<Output<PushPull>>;
pub type Motor1PWM = PA10<DefaultMode>;
pub type Motor2Phase = PA15<Output<PushPull>>;
pub type Motor2PWM = PA11<DefaultMode>;

// Stepper Motors
pub type Stepper1Ref = PC7<DefaultMode>;
pub type Stepper1Fault = PC13<Input<Floating>>;
pub type Stepper1Standby = PB5<Output<PushPull>>;
pub type Stepper1Enable = PC14<Output<PushPull>>;
pub type Stepper1Step = PD3<Output<PushPull>>;
pub type Stepper1Dir = PB3<Output<PushPull>>;

pub type Stepper2Ref = PB1<DefaultMode>;
pub type Stepper2Fault = PB10<Input<Floating>>;
pub type Stepper2Standby = PB0<Output<PushPull>>;
pub type Stepper2Enable = PB2<Output<PushPull>>;
pub type Stepper2Step = PA6<Output<PushPull>>;
pub type Stepper2Dir = PA7<Output<PushPull>>;

pub type Stepper3Ref = PA8<DefaultMode>;
pub type Stepper3Fault = PC6<Input<Floating>>;
pub type Stepper3Standby = PB15<Output<PushPull>>;
pub type Stepper3Enable = PA9<Output<PushPull>>;
pub type Stepper3Step = PB14<Output<PushPull>>;
pub type Stepper3Dir = PB13<Output<PushPull>>;

// Power switches
pub type Switch1Enable = PB12<Output<PushPull>>;
pub type Switch1Sense = PB11<Input<Floating>>;

pub type Switch2Enable = PF2<Output<PushPull>>;
pub type Switch2Sense = PF1<Input<Floating>>;

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
    pub motor1_phase: Motor1Phase,
    pub motor1_pwm: Motor1PWM,
    pub motor2_phase: Motor2Phase,
    pub motor2_pwm: Motor2PWM,

    // Stepper Motors
    pub stepper1_ref: Stepper1Ref,
    pub stepper1_fault: Stepper1Fault,
    pub stepper1_standby: Stepper1Standby,
    pub stepper1_enable: Stepper1Enable,
    pub stepper1_step: Stepper1Step,
    pub stepper1_dir: Stepper1Dir,

    pub stepper2_ref: Stepper2Ref,
    pub stepper2_fault: Stepper2Fault,
    pub stepper2_standby: Stepper2Standby,
    pub stepper2_enable: Stepper2Enable,
    pub stepper2_step: Stepper2Step,
    pub stepper2_dir: Stepper2Dir,

    pub stepper3_ref: Stepper3Ref,
    pub stepper3_fault: Stepper3Fault,
    pub stepper3_standby: Stepper3Standby,
    pub stepper3_enable: Stepper3Enable,
    pub stepper3_step: Stepper3Step,
    pub stepper3_dir: Stepper3Dir,

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
            motor1_phase: port_a.pa12.into(),
            motor2_phase: port_a.pa15.into(),
            motor1_pwm: port_a.pa10,
            motor2_pwm: port_a.pa11,

            // Stepper Motors
            stepper1_ref: port_c.pc7,
            stepper1_fault: port_c.pc13.into(),
            stepper1_standby: port_b.pb5.into(),
            stepper1_enable: port_c.pc14.into(),
            stepper1_step: port_d.pd3.into(),
            stepper1_dir: port_b.pb3.into(),

            stepper2_ref: port_b.pb1,
            stepper2_fault: port_b.pb10.into(),
            stepper2_standby: port_b.pb0.into(),
            stepper2_enable: port_b.pb2.into(),
            stepper2_step: port_a.pa6.into(),
            stepper2_dir: port_a.pa7.into(),

            stepper3_ref: port_a.pa8,
            stepper3_fault: port_c.pc6.into(),
            stepper3_standby: port_b.pb15.into(),
            stepper3_enable: port_a.pa9.into(),
            stepper3_step: port_b.pb14.into(),
            stepper3_dir: port_b.pb13.into(),

            // Power switches
            switch1_enable: port_b.pb12.into(),
            switch1_sense: port_b.pb11.into(),

            switch2_enable: port_f.pf2.into(),
            switch2_sense: port_f.pf1.into(),

            switch3_enable: port_f.pf3.into(),
            switch3_sense: port_c.pc15.into(),
        }
    }
}
