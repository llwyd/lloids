use nannou::prelude::*;
use crate::angle;
    
#[derive(Copy,Clone,Debug,PartialEq)]
enum State{
    Idle,
    TurningH,
    TurningV,
    TurningHarderH,
    TurningHarderV,
}

#[derive(Copy, Clone)]
pub struct Bird{
    xy: Point2,
    angle: f32,
    sep_angle: f32,
    align_angle: f32,
    coh_angle: f32,
    sep_changed: bool,
    coh_changed: bool,
    state:State,
    turn_angle:f32,
    avg_sep_angle:f32,
    avg_coh_angle:f32,
}

impl Bird{
    const BIRD_HEIGHT:f32 = 30.0;
    const BIRD_WIDTH_2:f32 = 10.0;

    const EDGE_BLEED:f32 = 50.0;

    const BIRD_REGION_RADIUS:f32 = 225.0; 
    const BIRD_SEPARATION_RADIUS:f32 = 30.0;

    const SPEED_GAIN:f32 = 1.4;

    const SEP_SPEED_MIN:f32 = 1.25 * Self::SPEED_GAIN;
    const SEP_SPEED_MAX:f32 = 2.5 * Self::SPEED_GAIN;
    
    const COH_SPEED_MIN:f32 = 0.5 * Self::SPEED_GAIN;
    const COH_SPEED_MAX:f32 = 1.5 * Self::SPEED_GAIN;

    const BIRD_SPEED_MIN:f32 = 1.0 * Self::SPEED_GAIN;
    const BIRD_SPEED_MAX:f32 = 7.5 * Self::SPEED_GAIN;

    /* NOTE: Radians */
    const SEP_ANGLE:f32 = 0.00625 * 1.8;
    const COH_ANGLE:f32 = 0.00005625 * 3.0;
    const ALIGNMENT_GAIN:f32 = 0.0275;

    const ALIGNMENT_INITIAL:f32 = 0.0;

    const TURN_GAIN:f32 = 0.020;

    const HARD_ANGLE_MULTIPLIER:f32 = 5.0;
    const HARD_ANGLE_SATURATION:f32 = 65.0;
    
    const NON_ZERO_ADJUST:f32 = 0.001;
    const DISTANCE_DECAY:f32 = 0.1;

    pub fn new(position:Point2, angle:f32) -> Bird{
        Bird{
            xy: position,
            angle: angle,
            sep_angle: angle,
            align_angle: Self::ALIGNMENT_INITIAL,
            coh_angle: angle,
            sep_changed: false,
            coh_changed: false,
            state: State::Idle,
            turn_angle: 0.0,
            avg_sep_angle: 0.0,
            avg_coh_angle: 0.0,
        }
    }

    pub fn angle(&self) -> f32{
        self.angle
    }

    pub fn set_rotation(&mut self, new_rotation:f32){
        self.angle = new_rotation;
    }
    
    pub fn set_separation(&mut self, new_rotation:f32, new_angle:f32){
        self.sep_angle = new_rotation;
        self.avg_sep_angle = new_angle;
        self.sep_changed = true;
    }
    
    pub fn get_separation(&self) -> f32{
        self.sep_angle
    }
    
    pub fn get_alignment(&self) -> f32{
        self.align_angle
    }
    
    pub fn get_cohesion(&self) -> f32{
        self.coh_angle
    }
    
    pub fn set_alignment(&mut self, new_rotation:f32){
        self.align_angle = new_rotation;
    }
    
    pub fn set_cohesion(&mut self, new_rotation:f32, new_angle:f32){
        self.coh_angle = new_rotation;
        self.avg_coh_angle = new_angle;
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
    
    pub fn get_position(&self) -> &Point2{
        &self.xy
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
            .points(pt2(Self::BIRD_HEIGHT / 2.0, 0.0),pt2(-Self::BIRD_HEIGHT / 2.0, -Self::BIRD_WIDTH_2),pt2(-Self::BIRD_HEIGHT / 2.0,Self::BIRD_WIDTH_2))
            .x_y(self.xy.x, self.xy.y)
            .rotate(self.angle)
            .color(WHITE);
    }

    pub fn update(&mut self, win: &Rect<f32>, inner: &Rect<f32>, inner_hard: &Rect<f32>)
    {
        assert!(self.angle >= 0.0);

        let mut sep_angle = self.sep_angle;
        let mut coh_angle = self.coh_angle;
        let mut align_gain = Self::ALIGNMENT_GAIN;


        let near_edge = self.is_near_edge(inner);

        if near_edge 
        {
            let dist = self.distance_outside(inner); 
            let reduct = (dist * -Self::DISTANCE_DECAY).exp();
            sep_angle *= reduct;
            coh_angle *= reduct;
            align_gain *= reduct;
        }

        /* Separation */
        if self.sep_changed{
            self.apply_separation(sep_angle, Self::SEP_ANGLE, Self::SEP_SPEED_MIN , Self::SEP_SPEED_MAX, true);
            self.sep_changed = false;
        }
        
        /* Cohesion */
        if self.coh_changed{
            self.apply_cohesion(coh_angle, Self::COH_ANGLE, Self::COH_SPEED_MIN, Self::COH_SPEED_MAX, true);
            self.coh_changed = false;
        }
        
        /* Adjust Alignment */
        self.angle += self.align_angle * align_gain;
        self.angle = angle::wrap(self.angle);
        
        assert!(self.angle != std::f32::INFINITY);
        assert!(self.angle != std::f32::NEG_INFINITY);
        assert!(self.angle >= 0.0);

        self.move_rnd(Self::BIRD_SPEED_MIN, Self::BIRD_SPEED_MAX); 

        self.state_machine(win, inner, inner_hard);
        self.screen_wrap(win);
    }

    fn state_machine(&mut self, _win: &Rect<f32>, inner: &Rect<f32>, inner_hard: &Rect<f32>)
    {
        match self.state{
            State::Idle =>
            {
                /* If near edge, then calculate turning angle
                 * and change state */
                if self.h_is_near_edge(inner)
                {
                    if self.xy.x > inner.right() as f32
                    {
                        let mut angle = self.angle;
                        angle = angle::wrap(angle);
                        if angle == std::f32::consts::PI
                        {
                            angle -= Self::NON_ZERO_ADJUST;
                        }
                        let turn_angle = std::f32::consts::PI - angle;
                        assert!(turn_angle != 0.0);
                        assert!(turn_angle >= -std::f32::consts::PI);
                        assert!(turn_angle <= std::f32::consts::PI);
                        self.turn_angle = turn_angle * Self::TURN_GAIN;
                    }
                    else if self.xy.x < inner.left() as f32
                    {
                        let mut angle = self.angle - (std::f32::consts::PI);
                        angle = angle::wrap(angle);
                        if angle == std::f32::consts::PI
                        {
                            angle -= Self::NON_ZERO_ADJUST;
                        }
                        
                        let turn_angle = std::f32::consts::PI - angle;
                        assert!(turn_angle != 0.0);
                        assert!(turn_angle >= -std::f32::consts::PI);
                        assert!(turn_angle <= std::f32::consts::PI);
                        self.turn_angle = turn_angle * Self::TURN_GAIN;

                    }
                    else
                    {
                        assert!(false);
                    }
                    self.state = State::TurningH;
                }
                else if self.v_is_near_edge(inner)
                {
                    if self.xy.y > inner.top() as f32
                    {
                        let mut angle = self.angle - (std::f32::consts::PI / 2.0);
                        angle = angle::wrap(angle);
                        if angle == std::f32::consts::PI
                        {
                            angle -= Self::NON_ZERO_ADJUST;
                        }
                        let turn_angle = std::f32::consts::PI - angle;
                        assert!(turn_angle != 0.0);
                        assert!(turn_angle >= -180.0);
                        assert!(turn_angle <= 180.0);
                        self.turn_angle = turn_angle * Self::TURN_GAIN;
                    }
                    else if self.xy.y < inner.bottom() as f32
                    {
                        let mut angle = self.angle - (std::f32::consts::PI * 1.5);
                        angle = angle::wrap(angle);
                        if angle == std::f32::consts::PI
                        {
                            angle -= Self::NON_ZERO_ADJUST;
                        }
                        
                        let turn_angle = std::f32::consts::PI - angle;
                        assert!(turn_angle != 0.0);
                        assert!(turn_angle >= -180.0);
                        assert!(turn_angle <= 180.0);
                        self.turn_angle = turn_angle * Self::TURN_GAIN;
                    }
                    else
                    {
                        assert!(false);
                    }

                    self.state = State::TurningV;
                }
            },
            State::TurningH =>
            {
                
                self.angle += self.turn_angle;
                self.angle = angle::wrap(self.angle);
                self.move_rnd(Self::BIRD_SPEED_MIN * 0.5, Self::BIRD_SPEED_MAX * 0.5); 

                if !self.h_is_near_edge(inner)
                {
                    self.state = State::Idle;
                }
                else if self.h_is_near_edge(inner_hard)
                {
                    self.state = State::TurningHarderH;
                }
                else if self.v_is_near_edge(inner_hard)
                {
                    self.state = State::TurningHarderV;
                }
            },
            State::TurningV =>
            {
                
                self.angle += self.turn_angle;
                self.angle = angle::wrap(self.angle);
                self.move_rnd(Self::BIRD_SPEED_MIN * 0.5, Self::BIRD_SPEED_MAX * 0.5); 

                if !self.v_is_near_edge(inner)
                {
                    self.state = State::Idle;
                }
                else if self.h_is_near_edge(inner_hard)
                {
                    self.state = State::TurningHarderH;
                }
                else if self.v_is_near_edge(inner_hard)
                {
                    self.state = State::TurningHarderV;
                }
            },
            State::TurningHarderH =>
            {
                self.angle += self.saturate_angle(self.turn_angle * Self::HARD_ANGLE_MULTIPLIER, deg_to_rad(Self::HARD_ANGLE_SATURATION));
                self.angle = angle::wrap(self.angle);
                self.move_rnd(Self::BIRD_SPEED_MIN, Self::BIRD_SPEED_MAX); 

                if !self.h_is_near_edge(inner_hard)
                {
                    self.state = State::Idle;
                }
            },
            State::TurningHarderV =>
            {
                self.angle += self.saturate_angle(self.turn_angle * Self::HARD_ANGLE_MULTIPLIER, deg_to_rad(Self::HARD_ANGLE_SATURATION));
                self.angle = angle::wrap(self.angle);
                self.move_rnd(Self::BIRD_SPEED_MIN, Self::BIRD_SPEED_MAX); 

                if !self.v_is_near_edge(inner_hard)
                {
                    self.state = State::Idle;
                }
            },
        }

    }

    fn move_rnd(&mut self, lower_speed:f32, upper_speed:f32)
    {
        let mov_inc = random_range(lower_speed, upper_speed); 
        self.move_bird(mov_inc);
    }

    fn move_bird(&mut self, mov_inc:f32)
    {
        self.xy.x += mov_inc * self.angle.cos();
        self.xy.y += mov_inc * self.angle.sin();
    }
    
    fn move_bird_to_angle(&mut self, mov_inc:f32, angle:f32)
    {
        self.xy.x += mov_inc * angle.cos();
        self.xy.y += mov_inc * angle.sin();
    }

    fn saturate_angle(&self, angle:f32, limit:f32)->f32
    {
        let mut new_angle = angle;
        if angle >= limit
        {
            new_angle = limit;
        }
        else if angle <= -limit
        {
            new_angle = limit;
        }
        new_angle
    }
    
    pub fn apply_separation(&mut self, angle: f32, rot_angle: f32, lower_speed: f32, upper_speed: f32, randomise: bool){
        /* Randomise movement */
        let mov_inc:f32;
        if randomise{
            mov_inc = random_range(lower_speed, upper_speed); 
        }
        else
        {
            mov_inc = upper_speed;
        }
        let old_xy = self.xy;
        
        /* 1. Move bird in direction of separation angle */
        if self.state == State::Idle
        {
            self.move_bird_to_angle(mov_inc / 2.0, angle);
        }
        /* 2. Calculate how much bird should rotate away from the reference_bird */
        let angle_offset = 0.0 - self.avg_sep_angle;
        
        /* 3. rotate the original point */
        let rotated_position = self.rotate(old_xy, angle_offset);
        let norm_angle = angle::wrap( self.angle - self.avg_sep_angle );
        let delta:f32;

        /* 4. Determine whether to add or subtract an angle to turn away as appropriate */
        if rotated_position.y.is_positive()
        {
            if norm_angle > deg_to_rad(90.0) && norm_angle < deg_to_rad(270.0)
            {
                delta = -rot_angle;
            }
            else
            {
                delta = rot_angle;
            }
        }
        else
        {
            if norm_angle > deg_to_rad(90.0) && norm_angle < deg_to_rad(270.0)
            {
                delta = rot_angle;
            }
            else
            {
                delta = -rot_angle;
            }
        }

        //let delta = (self.xy.y - old_xy.y).atan2(self.xy.x - old_xy.x);
        self.angle += delta;
        self.angle = angle::wrap(self.angle);
        
        /* 1. Move bird in direction of angle */
        self.move_bird(mov_inc / 2.0);
    }
    
    pub fn apply_cohesion(&mut self, angle: f32, rot_angle: f32, lower_speed: f32, upper_speed: f32, randomise: bool){
        /* Randomise movement */
        let mov_inc:f32;
        if randomise{
            mov_inc = random_range(lower_speed, upper_speed); 
        }
        else
        {
            mov_inc = upper_speed;
        }
        let old_xy = self.xy;

        /* 1. Move bird in direction of cohesion angle */
        if self.state == State::Idle
        {
            self.move_bird_to_angle(mov_inc / 2.0, angle);
        }
        /* 2. Calculate how much bird should rotate away from the reference_bird */
        let angle_offset = 0.0 - self.avg_coh_angle;
        
        /* 3. rotate the original point */
        let rotated_position = self.rotate(old_xy, angle_offset);
        let norm_angle = angle::wrap( self.angle - self.avg_coh_angle );
        let delta:f32;

        /* 4. Determine whether to add or subtract an angle to turn away as appropriate */
        if rotated_position.y.is_positive()
        {
            if norm_angle > deg_to_rad(90.0) && norm_angle < deg_to_rad(270.0)
            {
                delta = rot_angle;
            }
            else
            {
                delta = -rot_angle;
            }
        }
        else
        {
            if norm_angle > deg_to_rad(90.0) && norm_angle < deg_to_rad(270.0)
            {
                delta = -rot_angle;
            }
            else
            {
                delta = rot_angle;
            }
        }

        //let delta = (self.xy.y - old_xy.y).atan2(self.xy.x - old_xy.x);
        self.angle += delta;
        self.angle = angle::wrap(self.angle);

        /* 1. Move bird in direction of angle */
        self.move_bird(mov_inc / 2.0);
    }

    fn is_near_edge(&self, inner: &Rect<f32>) -> bool
    {
        let mut near_edge = false;
        if  self.xy.x > inner.right() as f32 ||
            self.xy.x < inner.left() as f32 ||
            self.xy.y > inner.top() as f32 ||
            self.xy.y < inner.bottom() as f32 {
            
                near_edge = true;
        }
        near_edge
    }
    
    fn h_is_near_edge(&self, inner: &Rect<f32>) -> bool
    {
        let mut near_edge = false;
        if  self.xy.x > inner.right() as f32 ||
            self.xy.x < inner.left() as f32 {
                near_edge = true;
        }
        near_edge
    }
    
    fn v_is_near_edge(&self, inner: &Rect<f32>) -> bool
    {
        let mut near_edge = false;
        if  self.xy.y > inner.top() as f32 ||
            self.xy.y < inner.bottom() as f32 {   
                near_edge = true;
        }
        near_edge
    }

    fn screen_wrap(&mut self, win: &Rect<f32>){
        if self.xy.x >= win.right() + Self::EDGE_BLEED as f32{
            self.xy.x -= win.wh().x + Self::EDGE_BLEED;
        }
        else if self.xy.x <= win.left() -Self::EDGE_BLEED as f32{
            self.xy.x += win.wh().x + Self::EDGE_BLEED;
        }
        
        if self.xy.y >= win.top() + Self::EDGE_BLEED as f32{
            self.xy.y -= win.wh().y + Self::EDGE_BLEED;
        }
        else if self.xy.y <= win.bottom() - Self::EDGE_BLEED as f32{
            self.xy.y += win.wh().y + Self::EDGE_BLEED;
        } 
    }

    pub fn rotate(&self, source:Point2, angle: f32) -> Point2{
        let x = (source.x * angle.cos()) - (source.y * angle.sin());
        let y = (source.x * angle.sin()) + (source.y * angle.cos());
   
        pt2(x, y)
    }

    pub fn distance_outside(&self, boundary: &Rect<f32>) -> f32
    {
        let dist_x = (self.xy.x - boundary.top()).abs();
        let dist_y = (self.xy.y - boundary.right()).abs();

        let mut dist:f32 = dist_y;
        if dist_x > dist_y
        {
            dist = dist_x;
        }
        dist
    }
}

#[cfg(test)]
mod tests {
    use super::*;    
    const FLOAT_PRECISION:f32 = 0.00001;
    
    fn compare_floats(x:f32, y:f32, precision:f32)->bool{

        let delta = (x - y).abs();
        delta <= precision
    }
/*
    fn test_separation(init_position:Point2, _exp_position:Point2, bird_angle:f32, sep_angle:f32, exp_angle:f32)
    {
        let mut bird = Bird::new(init_position, bird_angle);

        assert_eq!(bird.position().x, init_position.x);
        assert_eq!(bird.position().y, init_position.y);
        assert_eq!(bird.angle(), bird_angle);
        assert_eq!(bird.get_separation(), bird_angle);
        assert_eq!(bird.get_alignment(), 0.0);
        assert_eq!(bird.get_cohesion(), bird_angle);

        let lower_speed = 1.0;
        let upper_speed = 1.0;
        let rotation_angle = deg_to_rad(1.0);

        bird.apply_separation(sep_angle, rotation_angle, lower_speed, upper_speed, false);

        let expected_position = pt2(init_position.x + (upper_speed* exp_angle.cos()), init_position.y + (upper_speed *exp_angle.sin()));
        assert!(compare_floats(bird.position().x, expected_position.x, FLOAT_PRECISION));
        assert!(compare_floats(bird.position().y, expected_position.y, FLOAT_PRECISION));
        assert!(compare_floats(bird.angle(), exp_angle, FLOAT_PRECISION));

    }


    #[test]
    fn init_bird(){
        let x = 0.0;
        let y = 0.0;
        let angle = 0.0;
        let bird = Bird::new(pt2(x, y), angle);

        assert_eq!(bird.position().x, x);
        assert_eq!(bird.position().y, y);
        assert_eq!(bird.angle(), angle);
        assert_eq!(bird.get_separation(), angle);
        assert_eq!(bird.get_alignment(), angle);
        assert_eq!(bird.get_cohesion(), angle);
    }

    #[test]
    fn init_bird_non_zero(){
        let x = 12.34;
        let y = 56.78;
        let angle = 91.011;
        let bird = Bird::new(pt2(x, y), angle);

        assert_eq!(bird.position().x, x);
        assert_eq!(bird.position().y, y);
        assert_eq!(bird.angle(), angle);
        assert_eq!(bird.get_separation(), angle);
        assert_eq!(bird.get_alignment(), 0.0);
        assert_eq!(bird.get_cohesion(), angle);
    }  
    
    #[test]
    fn apply_separation_east_zero_x(){
        test_separation(pt2(0.0, 0.0), pt2(1.0, 0.0), deg_to_rad(0.0), deg_to_rad(0.0), deg_to_rad(1.0)); 
        test_separation(pt2(0.0, 0.0), pt2(1.0, 0.0), deg_to_rad(45.0), deg_to_rad(0.0), deg_to_rad(46.0)); 
        test_separation(pt2(0.0, 0.0), pt2(1.0, 0.0), deg_to_rad(90.0), deg_to_rad(0.0), deg_to_rad(91.0)); 
        test_separation(pt2(0.0, 0.0), pt2(1.0, 0.0), deg_to_rad(135.0), deg_to_rad(0.0), deg_to_rad(134.0)); 
        test_separation(pt2(0.0, 0.0), pt2(1.0, 0.0), deg_to_rad(180.0), deg_to_rad(0.0), deg_to_rad(179.0)); 
        test_separation(pt2(0.0, 0.0), pt2(1.0, 0.0), deg_to_rad(225.0), deg_to_rad(0.0), deg_to_rad(224.0)); 
        test_separation(pt2(0.0, 0.0), pt2(1.0, 0.0), deg_to_rad(270.0), deg_to_rad(0.0), deg_to_rad(271.0)); 
        test_separation(pt2(0.0, 0.0), pt2(1.0, 0.0), deg_to_rad(315.0), deg_to_rad(0.0), deg_to_rad(316.0)); 
    }
    
    #[test]
    fn apply_separation_east_pos_x(){
        test_separation(pt2(1.0, 0.0), pt2(2.0, 0.0), deg_to_rad(0.0), deg_to_rad(0.0), deg_to_rad(1.0)); 
        test_separation(pt2(1.0, 0.0), pt2(2.0, 0.0), deg_to_rad(45.0), deg_to_rad(0.0), deg_to_rad(46.0)); 
        test_separation(pt2(1.0, 0.0), pt2(2.0, 0.0), deg_to_rad(90.0), deg_to_rad(0.0), deg_to_rad(91.0)); 
        test_separation(pt2(1.0, 0.0), pt2(2.0, 0.0), deg_to_rad(135.0), deg_to_rad(0.0), deg_to_rad(134.0)); 
        test_separation(pt2(1.0, 0.0), pt2(2.0, 0.0), deg_to_rad(180.0), deg_to_rad(0.0), deg_to_rad(179.0)); 
        test_separation(pt2(1.0, 0.0), pt2(2.0, 0.0), deg_to_rad(225.0), deg_to_rad(0.0), deg_to_rad(224.0)); 
        test_separation(pt2(1.0, 0.0), pt2(2.0, 0.0), deg_to_rad(270.0), deg_to_rad(0.0), deg_to_rad(271.0)); 
        test_separation(pt2(1.0, 0.0), pt2(2.0, 0.0), deg_to_rad(315.0), deg_to_rad(0.0), deg_to_rad(316.0)); 
    }
    
    #[test]
    fn apply_separation_east_neg_x(){
        test_separation(pt2(-1.0, 0.0), pt2(0.0, 0.0), deg_to_rad(0.0), deg_to_rad(0.0), deg_to_rad(1.0)); 
        test_separation(pt2(-1.0, 0.0), pt2(0.0, 0.0), deg_to_rad(45.0), deg_to_rad(0.0), deg_to_rad(46.0)); 
        test_separation(pt2(-1.0, 0.0), pt2(0.0, 0.0), deg_to_rad(90.0), deg_to_rad(0.0), deg_to_rad(91.0)); 
        test_separation(pt2(-1.0, 0.0), pt2(0.0, 0.0), deg_to_rad(135.0), deg_to_rad(0.0), deg_to_rad(134.0)); 
        test_separation(pt2(-1.0, 0.0), pt2(0.0, 0.0), deg_to_rad(180.0), deg_to_rad(0.0), deg_to_rad(179.0)); 
        test_separation(pt2(-1.0, 0.0), pt2(0.0, 0.0), deg_to_rad(225.0), deg_to_rad(0.0), deg_to_rad(224.0)); 
        test_separation(pt2(-1.0, 0.0), pt2(0.0, 0.0), deg_to_rad(270.0), deg_to_rad(0.0), deg_to_rad(271.0)); 
        test_separation(pt2(-1.0, 0.0), pt2(0.0, 0.0), deg_to_rad(315.0), deg_to_rad(0.0), deg_to_rad(316.0)); 
    }
    
    #[test]
    fn apply_separation_east_zero_y(){
        test_separation(pt2(0.0, 0.0), pt2(1.0, 0.0), deg_to_rad(0.0), deg_to_rad(0.0), deg_to_rad(1.0)); 
        test_separation(pt2(0.0, 0.0), pt2(1.0, 0.0), deg_to_rad(45.0), deg_to_rad(0.0), deg_to_rad(46.0)); 
        test_separation(pt2(0.0, 0.0), pt2(1.0, 0.0), deg_to_rad(90.0), deg_to_rad(0.0), deg_to_rad(91.0)); 
        test_separation(pt2(0.0, 0.0), pt2(1.0, 0.0), deg_to_rad(135.0), deg_to_rad(0.0), deg_to_rad(134.0)); 
        test_separation(pt2(0.0, 0.0), pt2(1.0, 0.0), deg_to_rad(180.0), deg_to_rad(0.0), deg_to_rad(179.0)); 
        test_separation(pt2(0.0, 0.0), pt2(1.0, 0.0), deg_to_rad(225.0), deg_to_rad(0.0), deg_to_rad(224.0)); 
        test_separation(pt2(0.0, 0.0), pt2(1.0, 0.0), deg_to_rad(270.0), deg_to_rad(0.0), deg_to_rad(271.0)); 
        test_separation(pt2(0.0, 0.0), pt2(1.0, 0.0), deg_to_rad(315.0), deg_to_rad(0.0), deg_to_rad(316.0)); 
    }
    
    #[test]
    fn apply_separation_east_pos_y(){
        test_separation(pt2(0.0, 1.0), pt2(1.0, 1.0), deg_to_rad(0.0), deg_to_rad(0.0), deg_to_rad(1.0)); 
        test_separation(pt2(0.0, 1.0), pt2(1.0, 1.0), deg_to_rad(45.0), deg_to_rad(0.0), deg_to_rad(46.0)); 
        test_separation(pt2(0.0, 1.0), pt2(1.0, 1.0), deg_to_rad(90.0), deg_to_rad(0.0), deg_to_rad(91.0)); 
        test_separation(pt2(0.0, 1.0), pt2(1.0, 1.0), deg_to_rad(135.0), deg_to_rad(0.0), deg_to_rad(134.0)); 
        test_separation(pt2(0.0, 1.0), pt2(1.0, 1.0), deg_to_rad(180.0), deg_to_rad(0.0), deg_to_rad(179.0)); 
        test_separation(pt2(0.0, 1.0), pt2(1.0, 1.0), deg_to_rad(225.0), deg_to_rad(0.0), deg_to_rad(224.0)); 
        test_separation(pt2(0.0, 1.0), pt2(1.0, 1.0), deg_to_rad(270.0), deg_to_rad(0.0), deg_to_rad(271.0)); 
        test_separation(pt2(0.0, 1.0), pt2(1.0, 1.0), deg_to_rad(315.0), deg_to_rad(0.0), deg_to_rad(316.0)); 
    }
    
    #[test]
    fn apply_separation_east_neg_y(){
        test_separation(pt2(0.0, -1.0), pt2(1.0, -1.0), deg_to_rad(0.0), deg_to_rad(0.0), deg_to_rad(359.0)); 
        test_separation(pt2(0.0, -1.0), pt2(1.0, -1.0), deg_to_rad(45.0), deg_to_rad(0.0), deg_to_rad(44.0)); 
        test_separation(pt2(0.0, -1.0), pt2(1.0, -1.0), deg_to_rad(90.0), deg_to_rad(0.0), deg_to_rad(89.0)); 
        test_separation(pt2(0.0, -1.0), pt2(1.0, -1.0), deg_to_rad(135.0), deg_to_rad(0.0), deg_to_rad(136.0)); 
        test_separation(pt2(0.0, -1.0), pt2(1.0, -1.0), deg_to_rad(180.0), deg_to_rad(0.0), deg_to_rad(181.0)); 
        test_separation(pt2(0.0, -1.0), pt2(1.0, -1.0), deg_to_rad(225.0), deg_to_rad(0.0), deg_to_rad(226.0)); 
        test_separation(pt2(0.0, -1.0), pt2(1.0, -1.0), deg_to_rad(270.0), deg_to_rad(0.0), deg_to_rad(269.0)); 
        test_separation(pt2(0.0, -1.0), pt2(1.0, -1.0), deg_to_rad(315.0), deg_to_rad(0.0), deg_to_rad(314.0)); 
    }
    
    #[test]
    fn apply_separation_pos_x_zero_y_45(){
        let x = 1.0;
        let y = 0.0;
        let angle = 45.0;

        let exp_x = x + deg_to_rad(angle).cos();
        let exp_y = y + deg_to_rad(angle).sin();
        
        test_separation(pt2(x, y), pt2(exp_x, exp_y), deg_to_rad(0.0), deg_to_rad(angle), deg_to_rad(359.0)); 
        test_separation(pt2(x, y), pt2(exp_x, exp_y), deg_to_rad(45.0), deg_to_rad(angle), deg_to_rad(44.0)); 
        test_separation(pt2(x, y), pt2(exp_x, exp_y), deg_to_rad(90.0), deg_to_rad(angle), deg_to_rad(89.0)); 
        test_separation(pt2(x, y), pt2(exp_x, exp_y), deg_to_rad(135.0), deg_to_rad(angle), deg_to_rad(136.0)); 
        test_separation(pt2(x, y), pt2(exp_x, exp_y), deg_to_rad(180.0), deg_to_rad(angle), deg_to_rad(181.0)); 
        test_separation(pt2(x, y), pt2(exp_x, exp_y), deg_to_rad(225.0), deg_to_rad(angle), deg_to_rad(226.0)); 
        test_separation(pt2(x, y), pt2(exp_x, exp_y), deg_to_rad(270.0), deg_to_rad(angle), deg_to_rad(269.0)); 
        test_separation(pt2(x, y), pt2(exp_x, exp_y), deg_to_rad(315.0), deg_to_rad(angle), deg_to_rad(314.0)); 
    }
    
    #[test]
    fn rotate_minus_90()
    {
        let x = 0.0;
        let y = 1.0;
        let angle = 0.0;
        let bird = Bird::new(pt2(x, y), angle);
        
        let pos = pt2(x, y);
        let new = bird.rotate(pos, deg_to_rad(-90.0));

        assert!(compare_floats(new.x, 1.0, FLOAT_PRECISION));
        assert!(compare_floats(new.y, 0.0, FLOAT_PRECISION));
    }
    
    #[test]
    fn rotate_plus_90()
    {
        let x = 0.0;
        let y = 1.0;
        let angle = 0.0;
        let bird = Bird::new(pt2(x, y), angle);
        
        let pos = pt2(x, y);
        let new = bird.rotate(pos, deg_to_rad(90.0));

        assert!(compare_floats(new.x, -1.0, FLOAT_PRECISION));
        assert!(compare_floats(new.y, 0.0, FLOAT_PRECISION));
    }
    */
}
