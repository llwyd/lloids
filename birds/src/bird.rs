use nannou::prelude::*;

#[derive(Copy, Clone)]
pub struct Bird{
    xy: Point2,
    angle: f32,
}

impl Bird{
    const MOV_INC:f32 = 1.0;
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
        self.xy.x += Self::MOV_INC * self.angle.sin();
        self.xy.y += Self::MOV_INC * self.angle.cos();

        self.xy.x %= win.right() as f32;
        self.xy.y %= win.top() as f32;

    }

    pub fn position(&self) -> Point2{
        self.xy
    }
}
