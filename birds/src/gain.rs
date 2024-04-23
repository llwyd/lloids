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
        let pre_inc_gain = self.gain;
        
        self.gain += Self::INC;

        if pre_inc_gain.is_sign_negative()
        {
            /* Clip */
            if self.gain >= 0.0
            {
                self.gain = 0.0 - std::f32::EPSILON;
            }
        }
    }

    pub fn decrement(&mut self){
        let pre_dec_gain = self.gain;
        
        self.gain -= Self::INC;

        if pre_dec_gain.is_sign_positive()
        {
            /* Clip */
            if self.gain <= 0.0
            {
                self.gain = 0.0 + std::f32::EPSILON;
            }
        }
    }
}

