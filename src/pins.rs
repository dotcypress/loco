use crate::hal::gpio::{gpioa::*, gpiob::*, gpioc::*};
use hal::gpio::{DefaultMode, OpenDrain, Output, PushPull, Input, PullUp};
use hal::prelude::*;
use hal::rcc::Rcc;
use hal::stm32::*;

/// I2C
pub type I2cScl = PB6<Output<OpenDrain>>;
pub type I2cSda = PA10<Output<OpenDrain>>;

// SWD
pub type SwdIo = PA13<DefaultMode>;
pub type SwdClk = PA14<DefaultMode>;

/// UART
pub type UartTx = PA2<DefaultMode>;
pub type UartRx = PA3<DefaultMode>;

/// SPI FLASH
pub type FlashSck = PB3<DefaultMode>;
pub type FlashMiso = PA11<DefaultMode>;
pub type FlashMosi = PA12<DefaultMode>;
pub type FlashSpiCs = SwdIo;

/// GPIO
pub type G1 = PA6<DefaultMode>;
pub type G2 = PA7<DefaultMode>;
pub type G3 = PB0<DefaultMode>;
pub type G4 = PB1<DefaultMode>;
pub type G5 = PA0<DefaultMode>;
pub type G6 = PA1<DefaultMode>;
pub type G7 = PA4<DefaultMode>;
pub type G8 = PA5<DefaultMode>;
pub type LedPin = PA5<Output<OpenDrain>>;

/// Input
pub type Btn1 = PB5<DefaultMode>;
pub type Btn2 = PB7<DefaultMode>;
pub type Enc1 = PA8<DefaultMode>;
pub type Enc2 = PA9<DefaultMode>;

// Motors
pub type M1En = PC15<Output<PushPull>>;
pub type M1Fault = PC6<Input<PullUp>>;
pub type M1Dir = PC14<Output<PushPull>>;
pub type M1Clk = PB9<Output<PushPull>>;

pub type M2En = PA15<Output<PushPull>>;
pub type M2Fault = PB2<Input<PullUp>>;
pub type M2Dir = PB4<Output<PushPull>>;
pub type M2Clk = PB8<Output<PushPull>>;

pub struct Pins {
    /// I2C
    pub i2c_scl: I2cScl,
    pub i2c_sda: I2cSda,

    // SWD
    pub swd_io: SwdIo,
    pub swd_clk: SwdClk,

    /// UART
    pub uart_tx: UartTx,
    pub uart_rx: UartRx,

    /// SPI FLASH
    pub flash_sck: FlashSck,
    pub flash_miso: FlashMiso,
    pub flash_mosi: FlashMosi,

    /// GPIO
    pub g1: G1,
    pub g2: G2,
    pub g3: G3,
    pub g4: G4,
    pub g5: G5,
    pub g6: G6,
    pub g7: G7,
    pub g8: G8,

    /// Input
    pub btn1: Btn1,
    pub btn2: Btn2,
    pub enc1: Enc1,
    pub enc2: Enc2,

    // Motors
    pub m1_en: M1En,
    pub m1_fault: M1Fault,
    pub m1_dir: M1Dir,
    pub m1_clk: M1Clk,

    pub m2_en: M2En,
    pub m2_fault: M2Fault,
    pub m2_dir: M2Dir,
    pub m2_clk: M2Clk,
}

impl Pins {
    pub fn new(gpioa: GPIOA, gpiob: GPIOB, gpioc: GPIOC, rcc: &mut Rcc) -> Self {
        let port_a = gpioa.split(rcc);
        let port_b = gpiob.split(rcc);
        let port_c = gpioc.split(rcc);

        Self {
            /// I2C
            i2c_scl: port_b.pb6.into_open_drain_output_in_state(PinState::High),
            i2c_sda: port_a.pa10.into_open_drain_output_in_state(PinState::High),

            /// SWD
            swd_io: port_a.pa13,
            swd_clk: port_a.pa14,

            /// UART
            uart_tx: port_a.pa2,
            uart_rx: port_a.pa3,

            /// SPI FLASH
            flash_sck: port_b.pb3,
            flash_miso: port_a.pa11,
            flash_mosi: port_a.pa12,

            /// GPIO
            g1: port_a.pa6,
            g2: port_a.pa7,
            g3: port_b.pb0,
            g4: port_b.pb1,
            g5: port_a.pa0,
            g6: port_a.pa1,
            g7: port_a.pa4,
            g8: port_a.pa5,

            /// Input
            btn1: port_b.pb5,
            btn2: port_b.pb7,
            enc1: port_a.pa8,
            enc2: port_a.pa9,

            // Motors
            m1_en: port_c.pc15.into(),
            m1_fault: port_c.pc6.into(),
            m1_dir: port_c.pc14.into(),
            m1_clk: port_b.pb9.into(),

            m2_en: port_a.pa15.into(),
            m2_fault: port_b.pb2.into(),
            m2_dir: port_b.pb4.into(),
            m2_clk: port_b.pb8.into(),
        }
    }
}
