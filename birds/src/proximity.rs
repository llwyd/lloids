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

#[derive(Copy, Clone)]
pub struct Proximity{
    settings:ProximitySettings,
    angle:f32, // measured angle
    alignment:f32, // average alignment
    changed:bool,
}

impl Proximity{
    pub fn new(settings:ProximitySettings, angle:f32, alignment:f32) -> Proximity{
        Proximity{
            settings:settings,
            angle:angle,
            alignment:alignment,
            changed:false,
        }
    }

    pub fn settings(&self) -> ProximitySettings{
        self.settings
    }

    pub fn update(&mut self, angle:f32, alignment:f32){
        self.angle = angle;
        self.alignment = alignment;
        self.changed = true;
    }

    pub fn angle(&self) -> f32{
        self.angle
    }
   
    pub fn attenuate_angle(&mut self, reduct: f32){
        self.angle *= reduct;
    }

    pub fn alignment(&self) -> f32{
        self.alignment
    }

    pub fn changed(&self) -> bool{
        self.changed
    }

    pub fn reset(&mut self){
        assert!(self.changed);
        self.changed = false;
    }
}

