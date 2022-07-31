use crate::*;
use core::mem::MaybeUninit;
use heapless::Deque;

#[derive(Clone, Copy, Debug)]
pub enum StepperJob {
    ChannelSync(usize),
    Spin(StepperSpin),
}

#[derive(Clone, Copy, Debug)]
pub struct StepperSpin {
    pub dir: Dir,
    pub pulses: u16,
    pub feed_rate: u8,
}

impl StepperSpin {
    pub fn new(pulses: u16, dir: Dir, feed_rate: u8) -> Self {
        Self {
            pulses,
            dir,
            feed_rate,
        }
    }
}

impl From<StepperSpin> for StepperJob {
    fn from(spin: StepperSpin) -> Self {
        StepperJob::Spin(spin)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Dir {
    #[default]
    CW,
    CCW,
}

impl Dir {
    pub fn reverse(self) -> Dir {
        match self {
            Dir::CW => Dir::CCW,
            Dir::CCW => Dir::CW,
        }
    }
}

impl From<Dir> for PinState {
    fn from(dir: Dir) -> PinState {
        match dir {
            Dir::CW => PinState::Low,
            Dir::CCW => PinState::High,
        }
    }
}

pub struct ESC<const CH: usize, const N: usize> {
    jobs: [MaybeUninit<Deque<StepperJob, N>>; CH],
    started: bool,
}

impl<const CH: usize, const N: usize> Default for ESC<CH, N> {
    fn default() -> Self {
        let mut jobs = [Self::INIT; CH];
        for queue in jobs.iter_mut() {
            queue.write(Deque::new());
        }
        Self {
            jobs,
            started: false,
        }
    }
}

impl<const CH: usize, const N: usize> ESC<CH, N> {
    const INIT: MaybeUninit<Deque<StepperJob, N>> = MaybeUninit::uninit();

    pub fn reset(&mut self) {
        self.started = false;
        for queue in &mut self.jobs {
            let queue = unsafe { &mut *queue.as_mut_ptr() };
            queue.clear();
        }
    }

    pub fn pause(&mut self) {
        self.started = false;
    }

    pub fn resume(&mut self) {
        self.started = true;
    }

    pub fn free_space(&self) -> usize {
        let mut res = usize::MAX;
        for queue in &self.jobs {
            let queue = unsafe { &*queue.as_ptr() };
            res = res.min(N - queue.len());
        }
        res
    }

    pub fn push_job<J: Into<StepperJob>>(
        &mut self,
        motor_idx: usize,
        job: J,
    ) -> Result<(), StepperJob> {
        let job = job.into();
        let queue = self.motor_queue(motor_idx).ok_or(job)?;

        queue.push_back(job)
    }

    pub fn poll(&mut self, motor_idx: usize) -> Option<StepperSpin> {
        if !self.started {
            return None;
        }
        match self.motor_queue(motor_idx)?.pop_front() {
            Some(StepperJob::Spin(job)) => Some(job),
            Some(StepperJob::ChannelSync(sync_idx)) => {
                let sync_queue = self.motor_queue(sync_idx)?;
                match sync_queue.front() {
                    None => {}
                    Some(StepperJob::ChannelSync(idx)) if sync_idx == *idx => {
                        sync_queue.pop_front();
                    }
                    _ => {
                        self.motor_queue(motor_idx)?
                            .push_front(StepperJob::ChannelSync(sync_idx))
                            .ok();
                    }
                }
                None
            }
            _ => None,
        }
    }

    fn motor_queue(&mut self, motor_idx: usize) -> Option<&mut Deque<StepperJob, N>> {
        if motor_idx >= self.jobs.len() {
            return None;
        }
        let queue = unsafe { &mut *self.jobs[motor_idx].as_mut_ptr() };
        Some(queue)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum StepperStatus {
    Busy,
    Idle,
}

pub struct Stepper<PP, DP, EP, const MAX_FEED_RATE: u8> {
    cnt: u8,
    pulses: u16,
    prescaler: u8,
    pulse_pin: PP,
    dir_pin: DP,
    en_pin: EP,
    reverse: bool,
}

impl<PP, DP, EP, const MAX_FEED_RATE: u8> Stepper<PP, DP, EP, MAX_FEED_RATE>
where
    PP: OutputPin,
    DP: OutputPin,
    EP: OutputPin,
{
    pub fn new(pulse_pin: PP, dir_pin: DP, en_pin: EP) -> Self {
        Self {
            pulses: 0,
            cnt: 0,
            prescaler: 0,
            reverse: false,
            pulse_pin,
            dir_pin,
            en_pin,
        }
    }

    pub fn set_reverse(&mut self, reverse: bool) {
        self.reverse = reverse;
    }

    pub fn load(&mut self, job: &StepperSpin) {
        self.pulses = job.pulses;
        self.prescaler = MAX_FEED_RATE.saturating_sub(job.feed_rate).max(1);
        self.cnt = 0;
        self.pulse_pin.set_low().ok();
        let dir = if self.reverse {
            job.dir.reverse()
        } else {
            job.dir
        };
        self.dir_pin.set_state(dir.into()).ok();
        self.en_pin.set_high().ok();
    }

    pub fn poll(&mut self) -> StepperStatus {
        if self.pulses == 0 {
            self.en_pin.set_low().ok();
            return StepperStatus::Idle;
        }

        if self.cnt >= self.prescaler {
            self.pulses -= 1;
            self.cnt = 0;
            self.pulse_pin.set_high().ok();
        } else {
            self.cnt += 1;
            self.pulse_pin.set_low().ok();
        }

        StepperStatus::Busy
    }
}
