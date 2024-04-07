
#[derive(Copy, Clone)]
pub struct Meta{
    iterations: u64,
}

impl Meta
{
    pub fn new() -> Meta{
        Meta{
            iterations: 0,
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
}
