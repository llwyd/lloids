use crate::gain::Gain;

#[derive(Copy, Clone)]
pub struct Speed{
    min:Gain,
    max:Gain,
    randomise:bool,
}

impl Speed{
    pub fn new(min:f32, max:f32, randomise: bool) -> Speed{
        Speed{
            min: Gain::new(min),
            max: Gain::new(max),
            randomise:randomise,
        }
    }

    pub fn min(&self)->f32{
        self.min.gain()
    }
   
    pub fn inc_min(&mut self){
        self.min.increment();
    }
    
    pub fn dec_min(&mut self){
        self.min.decrement();
    }
    
    pub fn inc_max(&mut self){
        self.max.increment();
    }
    
    pub fn dec_max(&mut self){
        self.max.decrement();
    }

    pub fn max(&self)->f32{
        self.max.gain()
    }

    pub fn randomise(&self)->bool{
        self.randomise
    }
}

