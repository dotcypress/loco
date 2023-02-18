use crate::*;
use hal::hal::PwmPin;
use heapless::Deque;

#[derive(Clone, Copy, Debug)]
pub enum Job {
    Sync(usize),
    Spin(Spin),
}

#[derive(Clone, Copy, Debug)]
pub struct Spin {
    pub pulses: i32,
    pub prescaler: u8,
}

impl Spin {
    pub fn new(pulses: i32, prescaler: u8) -> Self {
        Self { pulses, prescaler }
    }
}

impl From<Spin> for Job {
    fn from(spin: Spin) -> Self {
        Job::Spin(spin)
    }
}

pub struct StepperMotor<PP, DP, EP, SP, FP, REF, const Q: usize> {
    pulse_pin: PP,
    dir_pin: DP,
    enable_pin: EP,
    standby_pin: SP,
    fault_pin: FP,
    ref_pwm: REF,
    reverse: bool,
    position: i32,
    pulses: i32,
    cnt: u8,
    prescaler: u8,
    jobs: Deque<Job, Q>,
}

impl<PP, DP, EP, SP, FP, REF, const Q: usize> StepperMotor<PP, DP, EP, SP, FP, REF, Q>
where
    PP: OutputPin,
    DP: OutputPin,
    EP: OutputPin,
    SP: OutputPin,
    FP: InputPin,
    REF: PwmPin<Duty = u32>,
{
    pub fn new(
        pulse_pin: PP,
        dir_pin: DP,
        enable_pin: EP,
        standby_pin: SP,
        fault_pin: FP,
        ref_pwm: REF,
    ) -> Self {
        Self {
            pulse_pin,
            dir_pin,
            enable_pin,
            standby_pin,
            fault_pin,
            ref_pwm,
            position: 0,
            pulses: 0,
            cnt: 0,
            prescaler: 0,
            reverse: false,
            jobs: Deque::new(),
        }
    }

    pub fn fault_detected(&self) -> bool {
        self.fault_pin.is_low().unwrap_or(true)
    }

    pub fn set_reverse(&mut self, reverse: bool) {
        self.reverse = reverse;
    }

    pub fn on(&mut self) {
        self.pulse_pin.set_low().ok();
        self.dir_pin.set_high().ok();
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
        self.ref_pwm.enable();
        self.ref_pwm.set_duty(duty as _);
    }

    pub fn free_space(&self) -> usize {
        Q - self.jobs.len()
    }

    pub fn push_job<J: Into<Job>>(&mut self, job: J) -> Result<(), Job> {
        self.jobs.push_back(job.into())
    }

    pub fn poll(&mut self, epoch: usize) -> Status {
        if self.fault_detected() {
            return Status::Fault;
        }

        if self.pulses != 0 {
            if self.cnt >= self.prescaler {
                let delta = self.pulses.signum();
                self.position += delta;
                self.pulses -= delta;
                self.cnt = 0;
                self.pulse_pin.set_high().ok();
            } else {
                self.cnt += 1;
                self.pulse_pin.set_low().ok();
            }
            Status::Busy
        } else {
            loop {
                match self.jobs.front() {
                    Some(Job::Spin(spin)) => {
                        self.cnt = 0;
                        self.prescaler = spin.prescaler.max(1);
                        self.pulses = if self.reverse {
                            -spin.pulses
                        } else {
                            spin.pulses
                        };
                        if self.pulses.is_positive() {
                            self.dir_pin.set_low().ok();
                        } else {
                            self.dir_pin.set_high().ok();
                        }
                        self.pulse_pin.set_low().ok();
                        self.jobs.pop_front();
                        return Status::Busy;
                    }
                    Some(Job::Sync(e)) if *e == epoch => {
                        self.jobs.pop_front();
                        continue;
                    }
                    _ => return Status::Idle,
                }
            }
        }
    }
}
