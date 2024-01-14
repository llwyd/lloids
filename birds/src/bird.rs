use nannou::prelude::*;

#[derive(Copy, Clone)]
pub struct Bird{
    xy: Point2,
    angle: f32,
}

impl Bird{
    pub fn new() -> Bird{
        Bird{
            xy: pt2(0.0, 0.0),
            angle: 0.0,
        }
    }

    pub fn draw(&self, draw: &Draw)
    {
        draw.tri()
            .points(pt2(0.0,36.0),pt2(-15.0,0.0),pt2(15.0,0.0))
            .x_y(self.xy.x, self.xy.y)
            .rotate(self.angle)
            .color(WHITE);
    }

    pub fn update(&mut self, win: &Rect<f32>)
    {
        //self.xy.y =
    }

    pub fn position(&self) -> Point2{
        self.xy
    }
}
