#[derive(Copy, Clone)]
pub struct Gain{
    gain:f32,
}

impl Gain{
    const INC:f32 = 0.001;
    pub fn new(gain:f32) -> Gain{
        Gain{
            gain:gain,
        }
    }

    pub fn gain(&self) -> f32{
        self.gain
    }

    pub fn set(&mut self, gain:f32){
        self.gain = gain;
    }

    pub fn increment(&mut self){
        self.gain += Self::INC;
    }

    pub fn decrement(&mut self){
        self.gain -= Self::INC;
    }
}

