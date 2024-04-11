#[derive(Copy, Clone)]
pub struct Speed{
    min:f32,
    max:f32,
    randomise:bool,
}

impl Speed{
    pub fn new(min:f32, max:f32, randomise: bool) -> Speed{
        Speed{
            min: min,
            max: max,
            randomise:randomise,
        }
    }

    pub fn min(&self)->f32{
        self.min
    }
    
    pub fn max(&self)->f32{
        self.max
    }

    pub fn randomise(&self)->bool{
        self.randomise
    }
}

