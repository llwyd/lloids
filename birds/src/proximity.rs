use crate::speed::Speed;

#[derive(Copy, Clone)]
pub struct ProximitySettings{
    speed:Speed,
    delta:f32,
}

impl ProximitySettings{
    pub fn new(speed:Speed, delta: f32) -> ProximitySettings{
        ProximitySettings{
            speed:speed,
            delta:delta,
        }
    }

    pub fn speed(&self) -> Speed{
        self.speed
    }

    pub fn delta(&self) -> f32{
        self.delta
    }
}
