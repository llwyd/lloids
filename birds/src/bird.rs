use nannou::prelude::*;

#[derive(Copy, Clone)]
pub struct Bird{
    xy: Point2,
}

impl Bird{
    pub fn new() -> Bird{
        Bird{
            xy: pt2(0.0, 0.0),
        }
    }
    
    pub fn position(&self) -> Point2{
        self.xy
    }
}
