use nannou::prelude::*;

#[derive(Copy, Clone)]
pub struct Bird{
    xy: Point2,
    angle: f32,
}

impl Bird{
    const MOV_INC:f32 = 2.0;
    const BIRD_HEIGHT:f32 = 30.0;
    const BIRD_WIDTH_2:f32 = 10.0;
    const BIRD_REGION_RADIUS:f32 = 180.0;

    pub fn new(position:Point2) -> Bird{
        Bird{
            xy: position,
            angle: deg_to_rad(90.0),
        }
    }

    pub fn radius(&self) -> f32{
        Self::BIRD_REGION_RADIUS
    }

    pub fn draw_region(&self, draw: &Draw)
    {
        draw.ellipse()
            .color(GREY)
            .x_y(self.xy.x, self.xy.y)
            .w(Self::BIRD_REGION_RADIUS)
            .h(Self::BIRD_REGION_RADIUS);
    }

    pub fn draw(&self, draw: &Draw)
    {
        draw.tri()
            .points(pt2(0.0,Self::BIRD_HEIGHT / 2.0),pt2(-Self::BIRD_WIDTH_2, -Self::BIRD_HEIGHT / 2.0),pt2(Self::BIRD_WIDTH_2, -Self::BIRD_HEIGHT / 2.0))
            .x_y(self.xy.x, self.xy.y)
            .rotate(self.angle)
            .color(WHITE);
    }

    pub fn update(&mut self, win: &Rect<f32>)
    {
        self.xy.x += -Self::MOV_INC * self.angle.sin();
        self.xy.y += Self::MOV_INC * self.angle.cos();

        if self.xy.x >= win.right() as f32{
            self.xy.x -= win.wh().x;
        }
        else if self.xy.x <= win.left() as f32{
            self.xy.x += win.wh().x;
        }
        
        if self.xy.y >= win.top() as f32{
            self.xy.y -= win.wh().y;
        }
        else if self.xy.y <= win.bottom() as f32{
            self.xy.y += win.wh().y;
        } 
    }

    pub fn position(&self) -> Point2{
        self.xy
    }
}

