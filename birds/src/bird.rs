use nannou::prelude::*;
use crate::angle;
use crate::proximity::Proximity;
use crate::proximity::ProximitySettings;
use crate::gain::Gain;
use crate::speed::Speed;

#[derive(Copy,Clone,Debug,PartialEq)]
enum State{
    Idle,
    TurningH,
    TurningV,
    TurningHarderH,
    TurningHarderV,
}

const TRAIL_LEN:usize = 64;

/* struct used to initialise the bird */
#[derive(Copy, Clone)]
pub struct BirdConfig{
    pub separation:ProximitySettings,
    pub cohesion:ProximitySettings,
    pub alignment_gain:Gain,
    pub speed:Speed,
}

#[derive(Copy, Clone)]
pub struct Bird{
    xy: Point2,
    angle: f32,
    align_angle: f32,
    state:State,
    speed:Speed,
    turn_angle:f32,

    trail:[Point2; TRAIL_LEN],
    trail_pos:usize,
    
    separation:Proximity,
    cohesion:Proximity,
    alignment_gain:Gain,
}

impl Bird{
    const BIRD_HEIGHT:f32 = 30.0;
    const BIRD_WIDTH_2:f32 = 10.0;

    const EDGE_BLEED:f32 = 50.0;

    const BIRD_REGION_RADIUS:f32 = 225.0; 
    const BIRD_SEPARATION_RADIUS:f32 = 30.0;

    const ALIGNMENT_INITIAL:f32 = 0.0;

    const TURN_GAIN:f32 = 0.020;

    const HARD_ANGLE_MULTIPLIER:f32 = 5.0;
    const HARD_ANGLE_SATURATION:f32 = 65.0;
    
    const NON_ZERO_ADJUST:f32 = 0.001;
    const DISTANCE_DECAY:f32 = 0.1;

    pub fn new(position:Point2, angle:f32, config: BirdConfig) -> Bird{
        Bird{
            xy: position,
            angle: angle,
            align_angle: Self::ALIGNMENT_INITIAL,
            state: State::Idle,
            turn_angle: 0.0,
            trail: [position; TRAIL_LEN],
            trail_pos: 0,
            separation: Proximity::new(config.separation, angle, 0.0), 
            cohesion: Proximity::new(config.cohesion,angle, 0.0),
            alignment_gain: Gain::new(config.alignment_gain.gain()),
            speed: config.speed,
        }
    }

    pub fn angle(&self) -> f32{
        self.angle
    }

    pub fn refresh_settings(&mut self, config: &mut BirdConfig){
        self.separation.refresh_settings(&config.separation);
        self.cohesion.refresh_settings(&config.cohesion);
        self.alignment_gain.set(config.alignment_gain.gain());
        self.speed = config.speed;
    }

    pub fn set_rotation(&mut self, new_rotation:f32){
        self.angle = new_rotation;
    }
    
    pub fn set_separation(&mut self, new_rotation:f32, new_angle:f32){
        self.separation.update(new_rotation, new_angle);
    }
    
    pub fn get_separation(&self) -> f32{
        self.separation.angle()
    }
    
    pub fn get_alignment(&self) -> f32{
        self.align_angle
    }
    
    pub fn get_cohesion(&self) -> f32{
        self.cohesion.angle()
    }
    
    pub fn set_alignment(&mut self, new_rotation:f32){
        self.align_angle = new_rotation;
    }
    
    pub fn set_cohesion(&mut self, new_rotation:f32, new_angle:f32){
        self.cohesion.update(new_rotation, new_angle);
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

    pub fn draw_trail(&self, draw: &Draw)
    {
        let mut start_idx = self.trail_pos + 1;
        if start_idx == self.trail.len()
        {
            start_idx = 0;
        }

        let points = (0..self.trail.len() - 1).map(|_|{
            let point = self.trail[start_idx];
            start_idx += 1;
            if start_idx == self.trail.len()
            {
                start_idx = 0;
            }
            point
        });

        draw.polyline()
            .weight(1.5)
            .rgba8(100,100,100,20)
            .points(points);
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

        let mut align_gain = self.alignment_gain.gain();
        let near_edge = self.is_near_edge(inner);

        if near_edge 
        {
            let dist = self.distance_outside(inner); 
            let reduct = (dist * -Self::DISTANCE_DECAY).exp();
            self.separation.attenuate_angle(reduct);
            self.cohesion.attenuate_angle(reduct);
            align_gain *= reduct;
        }

        /* Separation */
        if self.separation.changed(){
            assert!(self.separation.settings().delta().is_positive());
            self.apply_proximity(self.separation);
            self.separation.reset();
        }
        
        /* Cohesion */
        if self.cohesion.changed(){
            assert!(self.cohesion.settings().delta().is_negative());
            self.apply_proximity(self.cohesion);
            self.cohesion.reset();
        }
        
        /* Adjust Alignment */
        self.angle += self.align_angle * align_gain;
        self.angle = angle::wrap(self.angle);
        
        assert!(self.angle != std::f32::INFINITY);
        assert!(self.angle != std::f32::NEG_INFINITY);
        assert!(self.angle >= 0.0);

        self.move_rnd(self.speed.min(), self.speed.max());

        self.state_machine(win, inner, inner_hard);
        self.screen_wrap(win);

        self.update_trail();
    }

    fn update_trail(&mut self)
    {
        self.trail[self.trail_pos] = self.xy;
        self.trail_pos += 1;
        self.trail_pos %= self.trail.len();

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
                self.move_rnd(self.speed.min() * 0.5, self.speed.max()* 0.5); 

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
                self.move_rnd(self.speed.min() * 0.5, self.speed.max() * 0.5); 

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
                self.move_rnd(self.speed.min(), self.speed.max()); 

                if !self.h_is_near_edge(inner_hard)
                {
                    self.state = State::Idle;
                }
            },
            State::TurningHarderV =>
            {
                self.angle += self.saturate_angle(self.turn_angle * Self::HARD_ANGLE_MULTIPLIER, deg_to_rad(Self::HARD_ANGLE_SATURATION));
                self.angle = angle::wrap(self.angle);
                self.move_rnd(self.speed.min(), self.speed.max());

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

    pub fn rotation_delta(&self, position: Point2, angle: f32, rot_angle: f32) -> f32
    {
        let delta:f32;
        if position.y.is_positive()
        {
            if angle > deg_to_rad(90.0) && angle < deg_to_rad(270.0)
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
            if angle > deg_to_rad(90.0) && angle < deg_to_rad(270.0)
            {
                delta = rot_angle;
            }
            else
            {
                delta = -rot_angle;
            }
        }
        delta
    }


    pub fn apply_proximity(&mut self, prox:Proximity)
    {
        assert!(prox.angle() >= -std::f32::consts::PI);
        assert!(prox.angle() <= std::f32::consts::PI);
        /* Randomise movement */
        let mov_inc:f32;
        
        if prox.settings().speed().randomise(){
            mov_inc = random_range(prox.settings().speed().min(), prox.settings().speed().max());
        }
        else
        {
            mov_inc = prox.settings().speed().max();
        }
        let old_xy = self.xy;
        
        /* 1. Move bird in direction of proximity angle */
        if self.state == State::Idle
        {
            self.move_bird_to_angle(mov_inc / 2.0, prox.angle());
        }
        /* 2. Calculate how much bird should rotate away from the reference_bird */
        let angle_offset = 0.0 - prox.alignment();
        
        /* 3. rotate the original point */
        let rotated_position = self.rotate(old_xy, angle_offset);
        let norm_angle = angle::wrap( self.angle - prox.alignment() );

        /* 4. Determine whether to add or subtract an angle to turn away as appropriate */
        let delta:f32 = self.rotation_delta(rotated_position, norm_angle, prox.settings().delta());

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
    use crate::speed::Speed;
    use crate::gain::Gain;
    use crate::proximity::ProximitySettings;
    use crate::proximity::Proximity;
    const FLOAT_PRECISION:f32 = 0.00001;
    
    fn compare_floats(x:f32, y:f32, precision:f32)->bool{

        let delta = (x - y).abs();
        delta <= precision
    }
    
    fn default_bird_config() -> BirdConfig{

        let speed = 1.0;
        let rotation_angle = deg_to_rad(1.0);

        BirdConfig{
            separation: ProximitySettings::new(Speed::new(speed, speed, false), rotation_angle),
            cohesion: ProximitySettings::new(Speed::new(speed, speed, false), -rotation_angle),
            alignment_gain: Gain::new(0.0),
            speed: Speed::new(speed, speed, false),
        }
    }

    fn test_separation(init_position:Point2, bird_angle:f32, sep_angle:f32, exp_angle:f32)
    {
        let speed = 1.0;
        let config = default_bird_config(); 
        let mut bird = Bird::new(init_position, bird_angle, config);

        assert_eq!(bird.position().x, init_position.x);
        assert_eq!(bird.position().y, init_position.y);
        assert_eq!(bird.angle(), bird_angle);
        assert_eq!(bird.get_separation(), bird_angle);
        assert_eq!(bird.get_alignment(), 0.0);
        assert_eq!(bird.get_cohesion(), bird_angle);

        let separation = Proximity::new(config.separation,sep_angle,0.0);

        bird.apply_proximity(separation);

        let position_step1 = pt2(init_position.x + (speed * 0.5 * sep_angle.cos()), init_position.y + (speed * 0.5 * sep_angle.sin()));
        let expected_position = pt2(position_step1.x + (speed * 0.5 * exp_angle.cos()), position_step1.y + (speed * 0.5 * exp_angle.sin()));
        println!("{:?}, {:?}", init_position, expected_position);
        assert!(compare_floats(bird.angle(), exp_angle, FLOAT_PRECISION));
        assert!(compare_floats(bird.position().x, expected_position.x, FLOAT_PRECISION));
        assert!(compare_floats(bird.position().y, expected_position.y, FLOAT_PRECISION));
    }
    
    fn test_cohesion(init_position:Point2, bird_angle:f32, sep_angle:f32, exp_angle:f32)
    {
        let speed = 1.0;
        let config = default_bird_config(); 
        let mut bird = Bird::new(init_position, bird_angle, config);

        assert_eq!(bird.position().x, init_position.x);
        assert_eq!(bird.position().y, init_position.y);
        assert_eq!(bird.angle(), bird_angle);
        assert_eq!(bird.get_separation(), bird_angle);
        assert_eq!(bird.get_alignment(), 0.0);
        assert_eq!(bird.get_cohesion(), bird_angle);

        let cohesion = Proximity::new(config.cohesion,sep_angle,0.0);

        bird.apply_proximity(cohesion);

        let position_step1 = pt2(init_position.x + (speed * 0.5 * sep_angle.cos()), init_position.y + (speed * 0.5 * sep_angle.sin()));
        let expected_position = pt2(position_step1.x + (speed * 0.5 * exp_angle.cos()), position_step1.y + (speed * 0.5 * exp_angle.sin()));
        println!("{:?}, {:?}", init_position, expected_position);
        assert!(compare_floats(bird.angle(), exp_angle, FLOAT_PRECISION));
        assert!(compare_floats(bird.position().x, expected_position.x, FLOAT_PRECISION));
        assert!(compare_floats(bird.position().y, expected_position.y, FLOAT_PRECISION));
    }
    
    fn test_rotation_delta(init_position:Point2, bird_angle:f32, rotation_angle:f32, exp_angle:f32)
    {
        let speed = 1.0;
        let config = BirdConfig{
            separation: ProximitySettings::new(Speed::new(speed,speed,false),rotation_angle),
            cohesion: ProximitySettings::new(Speed::new(speed,speed,false),-rotation_angle),
            alignment_gain: Gain::new(0.0),
            speed: Speed::new(speed, speed, false),
        };
        
        let bird = Bird::new(init_position, bird_angle, config);

        assert_eq!(bird.position().x, init_position.x);
        assert_eq!(bird.position().y, init_position.y);
        assert_eq!(bird.angle(), bird_angle);
        assert_eq!(bird.get_separation(), bird_angle);
        assert_eq!(bird.get_alignment(), 0.0);
        assert_eq!(bird.get_cohesion(), bird_angle);
        
        let delta = bird.rotation_delta(bird.position(), bird_angle, rotation_angle);
        assert!(compare_floats(delta, exp_angle, FLOAT_PRECISION));
    }

    #[test]
    fn init_bird(){
        let x = 0.0;
        let y = 0.0;
        let angle = 0.0;
        let config = default_bird_config();
        let bird = Bird::new(pt2(x, y), angle, config);

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
        let config = default_bird_config();
        let bird = Bird::new(pt2(x, y), angle, config);

        assert_eq!(bird.position().x, x);
        assert_eq!(bird.position().y, y);
        assert_eq!(bird.angle(), angle);
        assert_eq!(bird.get_separation(), angle);
        assert_eq!(bird.get_alignment(), 0.0);
        assert_eq!(bird.get_cohesion(), angle);
    }  
    
    #[test]
    fn apply_separation_quad_0(){
        for i in 0..91
        {
            let bird_angle = i as f32;
            for j in -180..181
            {
                let sep_angle = j as f32;
                let bird_angle_deg = deg_to_rad(bird_angle);
                /* initial position, bird angle, separation angle, expected angle */
                test_separation(pt2(0.0, 0.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_separation(pt2(1.0, 0.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_separation(pt2(1.0, 1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_separation(pt2(0.0, 1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_separation(pt2(-1.0, 0.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_separation(pt2(-1.0, -1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_separation(pt2(0.0, -1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_separation(pt2(1.0, -1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_separation(pt2(-1.0, 1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
            }
        }
    }
    
    #[test]
    fn apply_separation_quad_1(){
        for i in 91..181
        {
            let bird_angle = i as f32;
            for j in -180..181
            {
                let sep_angle = j as f32;
                let bird_angle_deg = deg_to_rad(bird_angle);
                /* initial position, bird angle, separation angle, expected angle */
                test_separation(pt2(0.0, 0.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_separation(pt2(1.0, 0.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_separation(pt2(1.0, 1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_separation(pt2(0.0, 1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_separation(pt2(-1.0, 0.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_separation(pt2(-1.0, -1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_separation(pt2(0.0, -1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_separation(pt2(1.0, -1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_separation(pt2(-1.0, 1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
            }
        }
    }

    #[test]
    fn apply_separation_quad_2(){
        for i in 181..270
        {
            let bird_angle = i as f32;
            for j in -180..181
            {
                let sep_angle = j as f32;
                let bird_angle_deg = deg_to_rad(bird_angle);
                /* initial position, bird angle, separation angle, expected angle */
                test_separation(pt2(0.0, 0.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_separation(pt2(1.0, 0.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_separation(pt2(1.0, 1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_separation(pt2(0.0, 1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_separation(pt2(-1.0, 0.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_separation(pt2(-1.0, -1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_separation(pt2(0.0, -1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_separation(pt2(1.0, -1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_separation(pt2(-1.0, 1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
            }
        }
    }
    
    #[test]
    fn apply_separation_quad_3(){
        for i in 270..360
        {
            let bird_angle = i as f32;
            for j in -180..181
            {
                let sep_angle = j as f32;
                let bird_angle_deg = deg_to_rad(bird_angle);
                /* initial position, bird angle, separation angle, expected angle */
                test_separation(pt2(0.0, 0.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_separation(pt2(1.0, 0.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_separation(pt2(1.0, 1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_separation(pt2(0.0, 1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_separation(pt2(-1.0, 0.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_separation(pt2(-1.0, -1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_separation(pt2(0.0, -1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_separation(pt2(1.0, -1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_separation(pt2(-1.0, 1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
            }
        }
    }
    
    #[test]
    fn apply_cohesion_quad_0(){
        for i in 0..91
        {
            let bird_angle = i as f32;
            for j in -180..181
            {
                let sep_angle = j as f32;
                let bird_angle_deg = deg_to_rad(bird_angle);
                /* initial position, bird angle, separation angle, expected angle */
                test_cohesion(pt2(0.0, 0.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_cohesion(pt2(1.0, 0.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_cohesion(pt2(1.0, 1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_cohesion(pt2(0.0, 1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_cohesion(pt2(-1.0, 0.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_cohesion(pt2(-1.0, -1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_cohesion(pt2(0.0, -1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_cohesion(pt2(1.0, -1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_cohesion(pt2(-1.0, 1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
            }
        }
    }
    
    #[test]
    fn apply_cohesion_quad_1(){
        for i in 91..181
        {
            let bird_angle = i as f32;
            for j in -180..181
            {
                let sep_angle = j as f32;
                let bird_angle_deg = deg_to_rad(bird_angle);
                /* initial position, bird angle, separation angle, expected angle */
                test_cohesion(pt2(0.0, 0.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_cohesion(pt2(1.0, 0.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_cohesion(pt2(1.0, 1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_cohesion(pt2(0.0, 1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_cohesion(pt2(-1.0, 0.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_cohesion(pt2(-1.0, -1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_cohesion(pt2(0.0, -1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_cohesion(pt2(1.0, -1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_cohesion(pt2(-1.0, 1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
            }
        }
    }

    #[test]
    fn apply_cohesion_quad_2(){
        for i in 181..270
        {
            let bird_angle = i as f32;
            for j in -180..181
            {
                let sep_angle = j as f32;
                let bird_angle_deg = deg_to_rad(bird_angle);
                /* initial position, bird angle, separation angle, expected angle */
                test_cohesion(pt2(0.0, 0.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_cohesion(pt2(1.0, 0.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_cohesion(pt2(1.0, 1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_cohesion(pt2(0.0, 1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_cohesion(pt2(-1.0, 0.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_cohesion(pt2(-1.0, -1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_cohesion(pt2(0.0, -1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_cohesion(pt2(1.0, -1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_cohesion(pt2(-1.0, 1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
            }
        }
    }
    
    #[test]
    fn apply_cohesion_quad_3(){
        for i in 270..360
        {
            let bird_angle = i as f32;
            for j in -180..181
            {
                let sep_angle = j as f32;
                let bird_angle_deg = deg_to_rad(bird_angle);
                /* initial position, bird angle, separation angle, expected angle */
                test_cohesion(pt2(0.0, 0.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_cohesion(pt2(1.0, 0.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_cohesion(pt2(1.0, 1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_cohesion(pt2(0.0, 1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_cohesion(pt2(-1.0, 0.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
                test_cohesion(pt2(-1.0, -1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_cohesion(pt2(0.0, -1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_cohesion(pt2(1.0, -1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(1.0)));
                test_cohesion(pt2(-1.0, 1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(bird_angle_deg + deg_to_rad(-1.0)));
            }
        }
    }

    #[test]
    fn apply_cohesion_bird_0_sep_0(){

        let bird_angle = 0.0;
        let sep_angle = 0.0;
        
        /* initial position, bird angle, separation angle, expected angle */
        test_cohesion(pt2(0.0, 0.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(deg_to_rad(-1.0)));
        test_cohesion(pt2(1.0, 0.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(deg_to_rad(-1.0)));
        test_cohesion(pt2(1.0, 1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(deg_to_rad(-1.0)));
        test_cohesion(pt2(0.0, 1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(deg_to_rad(-1.0)));
        test_cohesion(pt2(-1.0, 0.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(deg_to_rad(-1.0)));
        test_cohesion(pt2(-1.0, -1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(deg_to_rad(1.0)));
        test_cohesion(pt2(0.0, -1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(deg_to_rad(1.0)));
        test_cohesion(pt2(1.0, -1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(deg_to_rad(1.0)));
        test_cohesion(pt2(-1.0, 1.0), deg_to_rad(bird_angle), deg_to_rad(sep_angle), angle::wrap(deg_to_rad(-1.0)));
    } 
    
    #[test]
    fn rotation_delta_zero_bird_angle(){
    /* Positive angle would move bird away from the cluster */
        
        let bird_angle = 0.0;
        let rotation_angle = 1.0;

        test_rotation_delta(pt2(0.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
        test_rotation_delta(pt2(1.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
        test_rotation_delta(pt2(1.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
        test_rotation_delta(pt2(0.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
        test_rotation_delta(pt2(-1.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
        test_rotation_delta(pt2(-1.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
        test_rotation_delta(pt2(0.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
        test_rotation_delta(pt2(1.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
        test_rotation_delta(pt2(-1.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
    }
    
    #[test]
    fn rotation_delta_zero_bird_angle_neg_rot(){
    /* Positive angle would move bird away from the cluster */
        
        let bird_angle = 0.0;
        let rotation_angle = -1.0;

        test_rotation_delta(pt2(0.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
        test_rotation_delta(pt2(1.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
        test_rotation_delta(pt2(1.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
        test_rotation_delta(pt2(0.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
        test_rotation_delta(pt2(-1.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
        test_rotation_delta(pt2(-1.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
        test_rotation_delta(pt2(0.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
        test_rotation_delta(pt2(1.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
        test_rotation_delta(pt2(-1.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
    }
    
    #[test]
    fn rotation_delta_quad_0_positive_rot(){
    /* Positive angle would move bird away from the cluster */
        for i in 0..91
        {
            let bird_angle = i as f32;
            let rotation_angle = 1.0;

            test_rotation_delta(pt2(0.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(1.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(1.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(0.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(-1.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(-1.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(0.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(1.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(-1.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
        }
    }

    #[test]
    fn rotation_delta_quad_1_positive_rot(){
    /* Positive angle would move bird away from the cluster */
        for i in 91..181
        {
            let bird_angle = i as f32;
            let rotation_angle = 1.0;

            test_rotation_delta(pt2(0.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(1.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(1.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(0.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(-1.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(-1.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(0.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(1.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(-1.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
        }
    }
    
    #[test]
    fn rotation_delta_quad_2_positive_rot(){
    /* Positive angle would move bird away from the cluster */
        for i in 181..270
        {
            let bird_angle = i as f32;
            let rotation_angle = 1.0;

            test_rotation_delta(pt2(0.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(1.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(1.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(0.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(-1.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(-1.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(0.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(1.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(-1.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
        }
    }
    
    #[test]
    fn rotation_delta_quad_3_positive_rot(){
    /* Positive angle would move bird away from the cluster */
        for i in 270..360
        {
            let bird_angle = i as f32;
            let rotation_angle = 1.0;

            test_rotation_delta(pt2(0.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(1.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(1.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(0.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(-1.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(-1.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(0.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(1.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(-1.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
        }
    }
    
    #[test]
    fn rotation_delta_quad_0_negative_rot(){
    /* Positive angle would move bird away from the cluster */
        for i in 0..91
        {
            let bird_angle = i as f32;
            let rotation_angle = -1.0;

            test_rotation_delta(pt2(0.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(1.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(1.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(0.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(-1.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(-1.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(0.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(1.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(-1.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
        }
    }

    #[test]
    fn rotation_delta_quad_1_negative_rot(){
    /* Positive angle would move bird away from the cluster */
        for i in 91..181
        {
            let bird_angle = i as f32;
            let rotation_angle = -1.0;

            test_rotation_delta(pt2(0.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(1.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(1.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(0.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(-1.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(-1.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(0.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(1.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(-1.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
        }
    }
    
    #[test]
    fn rotation_delta_quad_2_negative_rot(){
    /* Positive angle would move bird away from the cluster */
        for i in 181..270
        {
            let bird_angle = i as f32;
            let rotation_angle = -1.0;

            test_rotation_delta(pt2(0.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(1.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(1.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(0.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(-1.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(-1.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(0.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(1.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(-1.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
        }
    }
    
    #[test]
    fn rotation_delta_quad_3_negative_rot(){
    /* Positive angle would move bird away from the cluster */
        for i in 270..360
        {
            let bird_angle = i as f32;
            let rotation_angle = -1.0;

            test_rotation_delta(pt2(0.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(1.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(1.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(0.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(-1.0,0.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
            test_rotation_delta(pt2(-1.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(0.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(1.0,-1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(1.0));
            test_rotation_delta(pt2(-1.0,1.0), deg_to_rad(bird_angle), deg_to_rad(rotation_angle), deg_to_rad(-1.0));
        }
    }

    #[test]
    fn rotate_minus_90()
    {
        let x = 0.0;
        let y = 1.0;
        let angle = 0.0;
        let config = default_bird_config();
        let bird = Bird::new(pt2(x, y), angle, config);
        
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
        let config = default_bird_config();
        let bird = Bird::new(pt2(x, y), angle, config);
        
        let pos = pt2(x, y);
        let new = bird.rotate(pos, deg_to_rad(90.0));

        assert!(compare_floats(new.x, -1.0, FLOAT_PRECISION));
        assert!(compare_floats(new.y, 0.0, FLOAT_PRECISION));
    }
}
