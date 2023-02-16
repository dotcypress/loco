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

#[derive(Clone, Copy, Debug)]
pub enum Status {
    Busy,
    Idle,
}

pub struct StepperMotor<PP, DP, EP, SP, REF, const Q: usize> {
    pulse_pin: PP,
    dir_pin: DP,
    en_pin: EP,
    standby_pin: SP,
    ref_pwm: REF,
    reverse: bool,
    position: i32,
    pulses: i32,
    cnt: u8,
    prescaler: u8,
    jobs: Deque<Job, Q>,
}

impl<PP, DP, EP, SP, REF, const Q: usize> StepperMotor<PP, DP, EP, SP, REF, Q>
where
    PP: OutputPin,
    DP: OutputPin,
    EP: OutputPin,
    SP: OutputPin,
    REF: PwmPin<Duty = u16>,
{
    pub fn new(
        pulse_pin: PP,
        dir_pin: DP,
        en_pin: EP,
        standby_pin: SP,
        ref_pwm: REF,
        reverse: bool,
    ) -> Self {
        Self {
            position: 0,
            pulses: 0,
            cnt: 0,
            prescaler: 0,
            reverse,
            pulse_pin,
            dir_pin,
            en_pin,
            standby_pin,
            ref_pwm,
            jobs: Deque::new(),
        }
    }

    pub fn switch_power(&mut self, on: bool) {
        if on {
            self.standby_pin.set_high().ok();
        } else {
            self.standby_pin.set_low().ok();
        }
    }

    pub fn set_tork(&mut self, tork: u16) {
        self.ref_pwm.set_duty(tork);
    }

    pub fn free_space(&self) -> usize {
        Q - self.jobs.len()
    }

    pub fn push_job<J: Into<Job>>(&mut self, job: J) -> Result<(), Job> {
        self.jobs.push_back(job.into())
    }

    pub fn poll(&mut self, epoch: usize) -> Status {
        if self.pulses == 0 {
            self.en_pin.set_low().ok();
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
                        self.en_pin.set_high().ok();

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
    }
}
