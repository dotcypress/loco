use crate::*;

pub enum DCMotor {
    MotorA,
    MotorB,
}
pub struct MotorController {
    phase_pins: (MotorAPhase, MotorBPhase),
    ref_pwm: PwmPin<RefTimer, timer::Channel1>,
    speed_pwm: (
        PwmPin<MotorTimer, timer::Channel4>,
        PwmPin<MotorTimer, timer::Channel3>,
    ),
    enable_pin: MotorEnable,
    standby_pin: MotorStandby,
    fault_pin: MotorFault,
    max_duty: u32,
}

impl MotorController {
    pub fn new(
        ref_pwm: PwmPin<RefTimer, timer::Channel1>,
        speed_pwm: (
            PwmPin<MotorTimer, timer::Channel4>,
            PwmPin<MotorTimer, timer::Channel3>,
        ),
        phase_pins: (MotorAPhase, MotorBPhase),
        enable_pin: MotorEnable,
        standby_pin: MotorStandby,
        fault_pin: MotorFault,
    ) -> Self {
        let max_duty = speed_pwm.0.get_max_duty() as u32;
        Self {
            speed_pwm,
            phase_pins,
            ref_pwm,
            enable_pin,
            standby_pin,
            fault_pin,
            max_duty,
        }
    }

    pub fn fault_detected(&self) -> bool {
        self.fault_pin.is_low().unwrap()
    }

    pub fn on(&mut self) {
        self.speed_pwm.0.set_duty(0);
        self.speed_pwm.0.enable();
        self.speed_pwm.1.set_duty(0);
        self.speed_pwm.1.enable();
        self.standby_pin.set_high().ok();
    }

    pub fn off(&mut self) {
        self.standby_pin.set_low().ok();
    }

    pub fn enable(&mut self) {
        self.enable_pin.set_high().ok();
    }

    pub fn disable(&mut self) {
        self.enable_pin.set_low().ok();
    }

    pub fn set_tork(&mut self, tork: u8) {
        let duty = self.ref_pwm.get_max_duty() as u32 * tork as u32 / 255;
        self.ref_pwm.set_duty(duty as _);
        self.ref_pwm.enable();
    }

    pub fn set_speed(&mut self, ch: DCMotor, val: i8) -> Status {
        let duty = self.max_duty * val.unsigned_abs() as u32 / 127;
        match ch {
            DCMotor::MotorA => {
                self.speed_pwm.0.set_duty(duty as _);
                self.phase_pins.0.set_state(val.is_negative().into()).ok();
            }
            DCMotor::MotorB => {
                self.speed_pwm.1.set_duty(duty as _);
                self.phase_pins.1.set_state(val.is_positive().into()).ok();
            }
        }
        if val == 0 {
            Status::Idle
        } else {
            Status::Busy
        }
    }
}
