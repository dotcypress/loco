use crate::*;

pub struct PowerSwitch<EP, SP> {
    enable_pin: EP,
    sense_pin: SP,
}

impl<EP, SP> PowerSwitch<EP, SP>
where
    EP: OutputPin,
    SP: InputPin,
{
    pub fn new(enable_pin: EP, sense_pin: SP) -> Self {
        Self {
            enable_pin,
            sense_pin,
        }
    }

    pub fn fault_detected(&self) -> bool {
        self.sense_pin.is_low().unwrap_or(true)
    }

    pub fn on(&mut self) {
        self.enable_pin.set_high().ok();
    }

    pub fn off(&mut self) {
        self.enable_pin.set_low().ok();
    }
}
