use std::time::{Instant, Duration};

#[derive(Copy, Clone)]
pub struct Meta{
    iterations: u64,
    runtime: Instant,
}

impl Meta
{
    pub fn new() -> Meta{
        Meta{
            iterations: 0,
            runtime: Instant::now(),
        }
    }

    pub fn update(&mut self)
    {
        self.iterations += 1;
    }

    pub fn iterations(&self) -> u64
    {
        self.iterations
    }

    pub fn runtime(&self) -> Duration
    {
        self.runtime.elapsed()
    }
}
