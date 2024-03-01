use nannou::prelude::*;
    
#[derive(Copy,Clone)]
enum State{
    Idle,
    TurningHorizontal,
    TurningVertical,
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
}

impl Bird{
    const BIRD_HEIGHT:f32 = 30.0;
    const BIRD_WIDTH_2:f32 = 10.0;

    const BIRD_REGION_RADIUS:f32 = 225.0; 
    const BIRD_SEPARATION_RADIUS:f32 = 30.0;

    const SEP_SPEED_MIN:f32 = 1.25;
    const SEP_SPEED_MAX:f32 = 2.5;
    
    const COH_SPEED_MIN:f32 = 0.5;
    const COH_SPEED_MAX:f32 = 1.5;

    const BIRD_SPEED_MIN:f32 = 1.0;
    const BIRD_SPEED_MAX:f32 = 7.5;

    /* NOTE: Radians */
    const SEP_ANGLE:f32 = 0.00625 * 0.25;
    const COH_ANGLE:f32 = 0.00005625 * 2.0;
    const ALIGNMENT_GAIN:f32 = 0.028;

    const ALIGNMENT_INITIAL:f32 = 0.0;
    const REDUCTION_FACTOR:f32 = 0.01;

    /* Degrees, confusing I know */
    const TURN_ANGLE:f32 = 1.0;
    const DECAY:f32 = 0.01;    
    const TURN_GAIN:f32 = 0.015;

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
        if near_edge {
            
            sep_angle *= Self::REDUCTION_FACTOR;
            coh_angle *= 10.0;
            align_gain *= 1.0;
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
        self.angle -= self.align_angle * align_gain;
        self.angle = self.wrap_angle(self.angle);
        
        assert!(self.angle != std::f32::INFINITY);
        assert!(self.angle != std::f32::NEG_INFINITY);
        
        self.angle = self.wrap_angle(self.angle);

        assert!(self.angle >= 0.0);
        self.move_rnd(Self::BIRD_SPEED_MIN, Self::BIRD_SPEED_MAX); 
        
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
                        if angle == 0.0
                        {
                            angle = 1.0;
                        }
                        let turn_angle = std::f32::consts::PI - angle;
                        assert!(turn_angle >= -180.0);
                        assert!(turn_angle <= 180.0);
                        self.turn_angle = turn_angle * Self::TURN_GAIN;
                    }
                    else if self.xy.x < inner.left() as f32
                    {
                        let mut angle = self.angle - (std::f32::consts::PI);
                        angle = self.wrap_angle(angle);
                        if angle == 0.0
                        {
                            angle = 1.0;
                        }
                        
                        let turn_angle = std::f32::consts::PI - angle;
                        assert!(turn_angle >= -180.0);
                        assert!(turn_angle <= 180.0);
                        self.turn_angle = turn_angle * Self::TURN_GAIN;

                    }
                    else
                    {
                        assert!(false);
                    }
                    self.state = State::TurningHorizontal;
                }
                else if self.v_is_near_edge(inner)
                {
                    if self.xy.y > inner.top() as f32
                    {
                        let mut angle = self.angle - (std::f32::consts::PI / 2.0);
                        if angle == 0.0
                        {
                            angle = 1.0;
                        }
                        let turn_angle = std::f32::consts::PI - angle;
                        assert!(turn_angle >= -180.0);
                        assert!(turn_angle <= 180.0);
                        self.turn_angle = turn_angle * Self::TURN_GAIN;
                    }
                    else if self.xy.y < inner.right() as f32
                    {
                        let mut angle = self.angle - (std::f32::consts::PI * 1.5);
                        angle = self.wrap_angle(angle);
                        if angle == 0.0
                        {
                            angle = 1.0;
                        }
                        
                        let turn_angle = std::f32::consts::PI - angle;
                        assert!(turn_angle >= -180.0);
                        assert!(turn_angle <= 180.0);
                        self.turn_angle = turn_angle * Self::TURN_GAIN;

                    }
                    else
                    {
                        assert!(false);
                    }

                    self.state = State::TurningVertical;
                }
            },
            State::TurningHorizontal =>
            {
                
                //self.angle += self.h_screen_edge(inner, self.turn_angle, Self::DECAY);
                self.angle += self.turn_angle;
                self.angle = self.wrap_angle(self.angle);
                self.move_rnd(Self::BIRD_SPEED_MIN * 0.25, Self::BIRD_SPEED_MAX * 0.25); 

                if !near_edge || self.is_near_edge(win)
                {
                    self.state = State::Idle;
                    self.turn_angle = 0.0;
                }
            },
            State::TurningVertical =>
            {
                //self.angle += self.v_screen_edge(inner, self.turn_angle, Self::DECAY); 
                self.angle += self.turn_angle;
                self.angle = self.wrap_angle(self.angle);
                self.move_rnd(Self::BIRD_SPEED_MIN * 0.25, Self::BIRD_SPEED_MAX * 0.25); 

                if !near_edge || self.is_near_edge(win)
                {
                    self.state = State::Idle;
                    self.turn_angle = 0.0;
                }
            },
        }


        /* Handle Screen Edge */
        /*
        if near_edge{
            self.angle += self.h_screen_edge(inner, deg_to_rad(Self::TURN_ANGLE), Self::DECAY);
            self.angle = self.wrap_angle(self.angle);
            self.move_rnd(Self::BIRD_SPEED_MIN * 0.25, Self::BIRD_SPEED_MAX * 0.25); 
            
            self.angle += self.v_screen_edge(inner, deg_to_rad(Self::TURN_ANGLE), Self::DECAY); 
            self.angle = self.wrap_angle(self.angle);
            self.move_rnd(Self::BIRD_SPEED_MIN * 0.25, Self::BIRD_SPEED_MAX * 0.25); 
        }
*/
/*
        /* Handle extreme edge of screen */
        let near_edge_hard = self.is_near_edge(inner_hard); 
        if near_edge_hard{
            self.angle += self.h_screen_edge(inner, deg_to_rad(Self::TURN_ANGLE * 10.0), Self::DECAY);
            self.angle = self.wrap_angle(self.angle);
            self.move_rnd(Self::BIRD_SPEED_MIN * 0.25, Self::BIRD_SPEED_MAX * 0.25); 
            
            self.angle += self.v_screen_edge(inner, deg_to_rad(Self::TURN_ANGLE * 10.0), Self::DECAY);
            self.angle = self.wrap_angle(self.angle);
            self.move_rnd(Self::BIRD_SPEED_MIN * 0.25, Self::BIRD_SPEED_MAX * 0.25); 
        }
*/
        self.screen_wrap(win);
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

    fn wrap_angle(&self, angle: f32) -> f32{
        let ref_angle = angle % (2.0 * std::f32::consts::PI);
        let mut wrapped_angle = ref_angle;
        
        if ref_angle < 0.0{
            wrapped_angle = ref_angle + ( 2.0 * std::f32::consts::PI );
        }
        else if ref_angle >= ( 2.0 * std::f32::consts::PI ){
            wrapped_angle = ref_angle - ( 2.0 * std::f32::consts::PI ); 
        }
        
        assert!(wrapped_angle >= 0.0);
        wrapped_angle
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

        /* 2. Calculate how much bird should rotate away from the reference_bird */
        let angle_offset = 0.0 - angle;
        
        /* 3. rotate the original point */
        let rotated_position = self.rotate(old_xy, angle_offset);
        let delta:f32;

        /* 4. Determine whether to add or subtract an angle to turn away as appropriate */
        if rotated_position.y.is_positive()
        {
            if self.angle > deg_to_rad(90.0) && self.angle < deg_to_rad(270.0)
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
            if self.angle > deg_to_rad(90.0) && self.angle < deg_to_rad(270.0)
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
        self.angle = self.wrap_angle(self.angle);
        
        /* 1. Move bird in direction of angle */
        self.move_bird(mov_inc);
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


        /* 2. Calculate how much bird should rotate away from the reference_bird */
        let angle_offset = 0.0 - angle;
        
        /* 3. rotate the original point */
        let rotated_position = self.rotate(old_xy, angle_offset);
        let delta:f32;

        /* 4. Determine whether to add or subtract an angle to turn away as appropriate */
        if rotated_position.y.is_positive()
        {
            if self.angle > deg_to_rad(90.0) && self.angle < deg_to_rad(270.0)
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
            if self.angle > deg_to_rad(90.0) && self.angle < deg_to_rad(270.0)
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
        self.angle = self.wrap_angle(self.angle);

        /* 1. Move bird in direction of angle */
        self.move_bird(mov_inc);
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

    fn v_screen_edge(&mut self, inner: &Rect<f32>, turn_angle:f32, decay: f32) -> f32
    {
        let mut turn = 1.0;
        let mut diff = 0.0;
    
        if self.xy.y > inner.top() as f32{
            let mut delta = self.xy.y - inner.top();
            assert!(delta >= 0.0);

            delta *= decay;
            let mut angle = self.angle - (std::f32::consts::PI / 2.0);
            angle = self.wrap_angle(angle);
            if rad_to_deg(angle) > 180.0{
                turn = -1.0;
            }
            diff = turn * turn_angle;
        }
        else if self.xy.y < inner.bottom() as f32{
            let mut delta = inner.bottom() - self.xy.y;
            assert!(delta >= 0.0);
            delta *= decay;
            let mut angle = self.angle - (std::f32::consts::PI * 1.5);
            angle = self.wrap_angle(angle);

            if rad_to_deg(angle) > 180.0{
                turn = -1.0;
            }
            diff = turn * turn_angle;
        }
        diff
    }

    fn h_screen_edge(&mut self, inner: &Rect<f32>, turn_angle:f32, decay:f32) -> f32
    {
        let mut turn = 1.0;
        let mut diff = 0.0;

        if self.xy.x > inner.right() as f32{
            let mut delta = self.xy.x - inner.right();
            assert!(delta >= 0.0);
            delta *= decay;
            let mut angle = self.angle;
            angle = self.wrap_angle(angle);

            if rad_to_deg(angle) > 180.0{
                turn = -1.0;
            }
            diff = turn * turn_angle;
        }
        else if self.xy.x < inner.left() as f32{
            let mut delta = inner.left() - self.xy.x;
            assert!(delta >= 0.0);
            let mut angle = self.angle - (std::f32::consts::PI);
            angle = self.wrap_angle(angle);

            if rad_to_deg(angle) > 180.0{
                turn = -1.0;
            }
            diff = turn * turn_angle;
        }
        diff
    }

    fn screen_wrap(&mut self, win: &Rect<f32>){
        if self.xy.x >= win.right() + 50.0 as f32{
            self.xy.x -= win.wh().x + 50.0;
        }
        else if self.xy.x <= win.left() -50.0 as f32{
            self.xy.x += win.wh().x + 50.0;
        }
        
        if self.xy.y >= win.top() +50.0 as f32{
            self.xy.y -= win.wh().y + 50.0;
        }
        else if self.xy.y <= win.bottom() - 50.0 as f32{
            self.xy.y += win.wh().y + 50.0;
        } 
    }

    pub fn rotate(&self, source:Point2, angle: f32) -> Point2{
        let x = (source.x * angle.cos()) - (source.y * angle.sin());
        let y = (source.x * angle.sin()) + (source.y * angle.cos());
   
        pt2(x, y)
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

    fn test_separation(init_position:Point2, exp_position:Point2, bird_angle:f32, dir_angle:f32, exp_angle:f32)
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

        bird.apply_separation(dir_angle, rotation_angle, lower_speed, upper_speed, false);

        assert!(compare_floats(bird.position().x, exp_position.x, FLOAT_PRECISION));
        assert!(compare_floats(bird.position().y, exp_position.y, FLOAT_PRECISION));
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
}
