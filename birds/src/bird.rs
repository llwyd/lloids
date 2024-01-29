use nannou::prelude::*;

#[derive(Copy, Clone)]
pub struct Bird{
    xy: Point2,
    angle: f32,
    sep_angle: f32,
    align_angle: f32,
    coh_angle: f32,
}

impl Bird{
    const MOV_INC:f32 = 0.2;
    const MOV_INC_MAX:f32 = 10.0;
    const MOV_INC_MIN:f32 = 0.01;
    const BIRD_HEIGHT:f32 = 30.0;
    const BIRD_WIDTH_2:f32 = 10.0;

    const BIRD_REGION_RADIUS:f32 = 140.0; 
    const BIRD_SEPARATION_RADIUS:f32 = 50.0;

    pub fn new(position:Point2, angle:f32) -> Bird{
        Bird{
            xy: position,
            angle: angle,
            sep_angle: 0.0,
            align_angle: 0.0,
            coh_angle: 0.0,
        }
    }

    pub fn angle(&self) -> f32{
        self.angle
    }

    pub fn set_rotation(&mut self, new_rotation:f32){
        self.angle = new_rotation;
    }
    
    pub fn set_separation(&mut self, new_rotation:f32){
        self.sep_angle = new_rotation;
    }
    
    pub fn set_alignment(&mut self, new_rotation:f32){
        self.align_angle = new_rotation;
    }
    
    pub fn set_cohesion(&mut self, new_rotation:f32){
        self.coh_angle = new_rotation;
    }

    pub fn radius(&self) -> f32{
        Self::BIRD_REGION_RADIUS
    }
    
    pub fn separation_radius(&self) -> f32{
        Self::BIRD_SEPARATION_RADIUS
    }

    pub fn draw_region(&self, draw: &Draw)
    {
        draw.ellipse()
            .color(GREY)
            .x_y(self.xy.x, self.xy.y)
            .w(Self::BIRD_REGION_RADIUS * 2.0)
            .h(Self::BIRD_REGION_RADIUS * 2.0);
    }
    
    pub fn draw_sep_region(&self, draw: &Draw)
    {
        draw.ellipse()
            .color(CYAN)
            .x_y(self.xy.x, self.xy.y)
            .w(Self::BIRD_SEPARATION_RADIUS * 2.0)
            .h(Self::BIRD_SEPARATION_RADIUS * 2.0);
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
        println!("Old Angle: {:?}", rad_to_deg(self.angle));
        let sep_angle = self.sep_angle * 0.25;
        let coh_angle = self.coh_angle * 0.25;

        let mov_inc = random_range(1.0, 5.0); 

        let mut sep = pt2(-mov_inc * sep_angle.sin(), mov_inc * sep_angle.cos());
//        let mut align = pt2(-mov_inc * self.align_angle.sin(), mov_inc * self.align_angle.cos());
        let mov_inc = random_range(Self::MOV_INC_MIN, 4.0); 
        let mut coh = pt2(-mov_inc * coh_angle.sin(), mov_inc * coh_angle.cos());


        /* Add new vectors */
        let mut new_xy = pt2(0.0, 0.0);

        new_xy.x = self.xy.x + sep.x  + coh.x;
        new_xy.y = self.xy.y + sep.y  + coh.y;

        self.angle += self.align_angle;
        println!("Sep: {:?}, Align: {:?}, Coh:{:?}", rad_to_deg(self.sep_angle), rad_to_deg(self.align_angle), rad_to_deg(self.coh_angle));
        assert!(self.angle != std::f32::INFINITY);
        assert!(self.angle != std::f32::NEG_INFINITY);
        //self.angle = new_xy.y.atan2(new_xy.x) - self.xy.y.atan2(self.xy.x);
        self.angle = (new_xy.y - self.xy.y).atan2(new_xy.x - self.xy.x);
        
        if self.angle < 0.0{
            self.angle = self.angle + ( 2.0 * std::f32::consts::PI );
        }
        else if self.angle >= ( 2.0 * std::f32::consts::PI ){
            self.angle = self.angle - ( 2.0 * std::f32::consts::PI ); 
        }

//        self.angle = (self.sep_angle + self.align_angle + self.coh_angle) / 1.0;

//        self.xy = new_xy;
//        self.xy.x += sep.x + align.x + coh.x;
//        self.xy.y += sep.y + align.y + coh.y;
        println!("New Angle: {:?}", rad_to_deg(self.angle));
        self.xy.x += -mov_inc * self.angle.sin();
        self.xy.y += mov_inc * self.angle.cos();

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

