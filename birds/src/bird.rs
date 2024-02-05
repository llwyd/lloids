use nannou::prelude::*;

#[derive(Copy, Clone)]
pub struct Bird{
    xy: Point2,
    angle: f32,
    sep_angle: f32,
    align_angle: f32,
    coh_angle: f32,
    sep: bool,
    coh: bool,
}

impl Bird{
    const MOV_INC:f32 = 0.2;
    const MOV_INC_MAX:f32 = 10.0;
    const MOV_INC_MIN:f32 = 0.01;
    const BIRD_HEIGHT:f32 = 30.0;
    const BIRD_WIDTH_2:f32 = 10.0;

    const BIRD_REGION_RADIUS:f32 = 200.0; 
    const BIRD_SEPARATION_RADIUS:f32 = 45.0;

    pub fn new(position:Point2, angle:f32) -> Bird{
        Bird{
            xy: position,
            angle: angle,
            sep_angle: angle,
            align_angle: 0.0,
            coh_angle: angle,
            sep: false,
            coh: false,
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
        self.sep = true;
    }
    
    pub fn set_alignment(&mut self, new_rotation:f32){
        self.align_angle = new_rotation;
    }
    
    pub fn set_cohesion(&mut self, new_rotation:f32){
        self.coh_angle = new_rotation;
        self.coh = true;
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

    pub fn update(&mut self, win: &Rect<f32>, inner: &Rect<f32>)
    {
        println!("Old Angle: {:?}", rad_to_deg(self.angle));
        let sep_angle = self.sep_angle * 1.0;
        let coh_angle = self.coh_angle * 1.0;

        let mov_inc = random_range(0.1, 1.0); 


        if self.sep{
            let old_xy = self.xy;
            self.xy.x += -mov_inc * sep_angle.sin();
            self.xy.y += mov_inc * sep_angle.cos();
            self.sep = false;
            let mut delta = (self.xy.y - old_xy.y).atan2(self.xy.x - old_xy.x);
            self.angle -= delta * 0.002;
        }
        let mov_inc = random_range(0.1, 1.0); 
        if self.coh{
            let old_xy = self.xy;
            self.xy.x += -mov_inc * coh_angle.sin();
            self.xy.y += mov_inc * coh_angle.cos();
            self.coh = false;
            let mut delta = (old_xy.y - self.xy.y).atan2(old_xy.x - self.xy.y);
            self.angle += delta * 0.002;
        }

        /* Handle Screen Edge */
        let turn_angle = deg_to_rad(60.0);
        let scaling = 0.0825;

        if self.xy.x >= inner.right() as f32{
            let mut turn = 1.0;
            let mut angle = self.angle - (std::f32::consts::PI * 1.5);
            if angle < 0.0{
                angle = angle + ( 2.0 * std::f32::consts::PI );
            }

            if rad_to_deg(angle) > 180.0{
                turn = -1.0;
            }
            self.angle += turn * turn_angle * scaling;
        }
        else if self.xy.x <= inner.left() as f32{
            let mut turn = 1.0;
            let mut angle = self.angle - (std::f32::consts::PI / 2.0);
            if angle < 0.0{
                angle = angle + ( 2.0 * std::f32::consts::PI );
            }

            if rad_to_deg(angle) > 180.0{
                turn = -1.0;
            }
            self.angle += turn * turn_angle * scaling;
        }
        
        if self.xy.y >= inner.top() as f32{
            let mut turn = 1.0;
            if rad_to_deg(self.angle) > 180.0{
                turn = -1.0;
            }
            self.angle += turn * turn_angle * scaling;
        }
        else if self.xy.y <= inner.bottom() as f32{
            let mut turn = 1.0;
            let mut angle = self.angle - std::f32::consts::PI;
            if angle < 0.0{
                angle = angle + ( 2.0 * std::f32::consts::PI );
            }

            if rad_to_deg(angle) > 180.0{
                turn = -1.0;
            }
            self.angle += turn * turn_angle * scaling;
        } 


        /* Add new vectors */
        let mut new_xy = pt2(0.0, 0.0);
        self.angle -= self.align_angle * 0.015;

        println!("Sep: {:?}, Align: {:?}, Coh:{:?}", rad_to_deg(self.sep_angle), rad_to_deg(self.align_angle), rad_to_deg(self.coh_angle));
        assert!(self.angle != std::f32::INFINITY);
        assert!(self.angle != std::f32::NEG_INFINITY);
        
        if self.angle < 0.0{
            self.angle = self.angle + ( 2.0 * std::f32::consts::PI );
        }
        else if self.angle >= ( 2.0 * std::f32::consts::PI ){
            self.angle = self.angle - ( 2.0 * std::f32::consts::PI ); 
        }

        println!("New Angle: {:?}", rad_to_deg(self.angle));
        let mov_inc = random_range(1.0, 2.0); 
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

