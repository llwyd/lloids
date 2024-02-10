use nannou::prelude::*;

#[derive(Copy, Clone)]
pub struct Bird{
    xy: Point2,
    angle: f32,
    sep_angle: f32,
    align_angle: f32,
    coh_angle: f32,
    sep_changed: bool,
    coh_changed: bool,
}

impl Bird{
    const BIRD_HEIGHT:f32 = 30.0;
    const BIRD_WIDTH_2:f32 = 10.0;

    const BIRD_REGION_RADIUS:f32 = 250.0; 
    const BIRD_SEPARATION_RADIUS:f32 = 50.0;

    const SEPARATION_GAIN:f32 = 0.024;
    const COHESION_GAIN:f32 = 0.005;
    const ALIGNMENT_GAIN:f32 = 0.05;

    const SEP_SPEED_MIN:f32 = 0.12;
    const SEP_SPEED_MAX:f32 = 1.2;
    
    const COH_SPEED_MIN:f32 = 0.1;
    const COH_SPEED_MAX:f32 = 1.0;

    const BIRD_SPEED_MIN:f32 = 1.0;
    const BIRD_SPEED_MAX:f32 = 5.0;
    

    const ALIGNMENT_INITIAL:f32 = 0.0;
    const REDUCTION_FACTOR:f32 = 0.5;

    const TURN_ANGLE:f32 = 45.0;
    const TURN_GAIN:f32 = 0.02;
    const DECAY:f32 = 0.025;

    pub fn new(position:Point2, angle:f32) -> Bird{
        Bird{
            xy: position,
            angle: angle,
            sep_angle: angle,
            align_angle: Self::ALIGNMENT_INITIAL,
            coh_angle: angle,
            sep_changed: false,
            coh_changed: false,
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
        self.sep_changed = true;
    }
    
    pub fn set_alignment(&mut self, new_rotation:f32){
        self.align_angle = new_rotation;
    }
    
    pub fn set_cohesion(&mut self, new_rotation:f32){
        self.coh_angle = new_rotation;
        self.coh_changed = true;
    }

    pub fn radius(&self) -> f32{
        Self::BIRD_REGION_RADIUS
    }
    
    pub fn separation_radius(&self) -> f32{
        Self::BIRD_SEPARATION_RADIUS
    }
    
    pub fn position(&self) -> Point2{
        self.xy
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
        assert!(self.angle >= 0.0);

        let mut sep_gain = Self::SEPARATION_GAIN;
        let mut coh_gain = Self::COHESION_GAIN;
        let mut align_gain = Self::ALIGNMENT_GAIN;

        let near_edge = self.is_near_edge(inner);
        if near_edge {
            
            sep_gain *= Self::REDUCTION_FACTOR;
            coh_gain *= Self::REDUCTION_FACTOR;
            align_gain *= Self::REDUCTION_FACTOR;
        }

        /* Separation */
        if self.sep_changed{
            let diff = self.spatial_awareness(self.sep_angle, sep_gain, Self::SEP_SPEED_MIN , Self::SEP_SPEED_MAX);
            self.angle -= diff;
            self.sep_changed = false;
        }
        
        /* Cohesion */
        if self.coh_changed{
            let diff = self.spatial_awareness(self.coh_angle, coh_gain, Self::COH_SPEED_MIN, Self::COH_SPEED_MAX);
            self.angle += diff;
            self.coh_changed = false;
        }

        /* Handle Screen Edge */
        if near_edge{
            self.angle += self.h_screen_edge(inner, deg_to_rad(Self::TURN_ANGLE), Self::TURN_GAIN);
            self.angle += self.v_screen_edge(inner, deg_to_rad(Self::TURN_ANGLE), Self::TURN_GAIN);
        
            self.angle = self.wrap_angle(self.angle);
        }

        /* Adjust Alignment */
        self.angle -= self.align_angle * align_gain;

        println!("Sep: {:?}, Align: {:?}, Coh:{:?}", rad_to_deg(self.sep_angle), rad_to_deg(self.align_angle), rad_to_deg(self.coh_angle));
        assert!(self.angle != std::f32::INFINITY);
        assert!(self.angle != std::f32::NEG_INFINITY);
        
        self.angle = self.wrap_angle(self.angle);

        println!("New Angle: {:?}", rad_to_deg(self.angle));
        assert!(self.angle >= 0.0);
        let mov_inc = random_range(Self::BIRD_SPEED_MIN, Self::BIRD_SPEED_MAX); 
        self.xy.x += -mov_inc * self.angle.sin();
        self.xy.y += mov_inc * self.angle.cos();

        self.screen_wrap(win);
    }

    fn wrap_angle(&self, angle: f32) -> f32{
        let mut wrapped_angle = angle;
        if angle < 0.0{
            wrapped_angle = angle + ( 2.0 * std::f32::consts::PI );
        }
        else if angle >= ( 2.0 * std::f32::consts::PI ){
            wrapped_angle = angle - ( 2.0 * std::f32::consts::PI ); 
        }

        assert!(wrapped_angle >= 0.0);
        wrapped_angle
    }

    fn spatial_awareness(&mut self, angle: f32, gain: f32, lower_speed: f32, upper_speed: f32) -> f32{
        /* Randomise movement */
        let mov_inc = random_range(lower_speed, upper_speed); 
        let old_xy = self.xy;
        self.xy.x += -mov_inc * angle.sin();
        self.xy.y += mov_inc * angle.cos();
        let delta = (self.xy.y - old_xy.y).atan2(self.xy.x - old_xy.x);
        delta * gain
    }

    fn is_near_edge(&self, inner: &Rect<f32>) -> bool
    {
        let mut near_edge = false;
        if  self.xy.x >= inner.right() as f32 ||
            self.xy.x <= inner.left() as f32 ||
            self.xy.y >= inner.top() as f32 ||
            self.xy.y <= inner.bottom() as f32 {
            
                near_edge = true;
        }
        near_edge
    }
    fn v_screen_edge(&mut self, inner: &Rect<f32>, turn_angle:f32, gain:f32) -> f32
    {
        let mut turn = 1.0;
        let mut diff = 0.0;
    
        if self.xy.y > inner.top() as f32{
            let mut delta = self.xy.y - inner.top();
            delta *= Self::DECAY;
            if rad_to_deg(self.angle) > 180.0{
                turn = -1.0;
            }
            diff = turn * turn_angle * gain * delta.exp();
        }
        else if self.xy.y < inner.bottom() as f32{
            let mut delta = inner.bottom() - self.xy.y;
            delta *= Self::DECAY;
            let mut angle = self.angle - std::f32::consts::PI;
            angle = self.wrap_angle(angle);

            if rad_to_deg(angle) > 180.0{
                turn = -1.0;
            }
            diff = turn * turn_angle * gain * delta.exp();
        }
        diff
    }

    fn h_screen_edge(&mut self, inner: &Rect<f32>, turn_angle:f32, gain:f32) -> f32
    {
        let mut turn = 1.0;
        let mut diff = 0.0;

        if self.xy.x > inner.right() as f32{
            let mut delta = self.xy.x - inner.right();
            delta *= Self::DECAY;
            let mut angle = self.angle - (std::f32::consts::PI * 1.5);
            angle = self.wrap_angle(angle);

            if rad_to_deg(angle) > 180.0{
                turn = -1.0;
            }
            diff = turn * turn_angle * gain * delta.exp();
        }
        else if self.xy.x < inner.left() as f32{
            let mut delta = inner.left() - self.xy.x;
            delta *= Self::DECAY;
            let mut angle = self.angle - (std::f32::consts::PI / 2.0);
            angle = self.wrap_angle(angle);

            if rad_to_deg(angle) > 180.0{
                turn = -1.0;
            }
            diff= turn * turn_angle * gain * delta.exp();
        }
        diff
    }

    fn screen_wrap(&mut self, win: &Rect<f32>){
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
}

