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

    const BIRD_REGION_RADIUS:f32 = 180.0; 
    const BIRD_SEPARATION_RADIUS:f32 = 65.0;

    const SEPARATION_GAIN:f32 = 0.05;
    const COHESION_GAIN:f32 = 0.025;
    const ALIGNMENT_GAIN:f32 = 0.050;

    const SEP_SPEED_MIN:f32 = 0.5;
    const SEP_SPEED_MAX:f32 = 1.0;
    
    const COH_SPEED_MIN:f32 = 0.1;
    const COH_SPEED_MAX:f32 = 0.5;

    const BIRD_SPEED_MIN:f32 = 1.0;
    const BIRD_SPEED_MAX:f32 = 5.0;

    const SEP_ANGLE:f32 = 0.5;
    const COH_ANGLE:f32 = 0.5;

    const ALIGNMENT_INITIAL:f32 = 0.0;
    const REDUCTION_FACTOR:f32 = 0.01;

    const TURN_ANGLE:f32 = 45.0;
    const TURN_GAIN:f32 = 0.02;
    const DECAY:f32 = 0.045;

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
            self.apply_separation(self.sep_angle, Self::SEP_ANGLE, sep_gain, Self::SEP_SPEED_MIN , Self::SEP_SPEED_MAX, true);
            //self.angle += diff;
            self.sep_changed = false;
        }
        
        /* Cohesion */
        if self.coh_changed{
            self.apply_cohesion(self.coh_angle, Self::COH_ANGLE, coh_gain, Self::COH_SPEED_MIN, Self::COH_SPEED_MAX, true);
            //self.angle -= diff;
            self.coh_changed = false;
        }
        
        /* Adjust Alignment */
        self.angle -= self.align_angle * align_gain;
        self.angle = self.wrap_angle(self.angle);
        
        println!("Sep: {:?}, Align: {:?}, Coh:{:?}", rad_to_deg(self.sep_angle), rad_to_deg(self.align_angle), rad_to_deg(self.coh_angle));
        assert!(self.angle != std::f32::INFINITY);
        assert!(self.angle != std::f32::NEG_INFINITY);
        
        self.angle = self.wrap_angle(self.angle);

        println!("New Angle: {:?}", rad_to_deg(self.angle));
        assert!(self.angle >= 0.0);
        let mov_inc = random_range(Self::BIRD_SPEED_MIN, Self::BIRD_SPEED_MAX); 
        self.xy.x += mov_inc * self.angle.cos();
        self.xy.y += mov_inc * self.angle.sin();

        /* Handle Screen Edge */
        if near_edge{
            self.angle += self.h_screen_edge(inner, deg_to_rad(Self::TURN_ANGLE), Self::TURN_GAIN);
            self.angle = self.wrap_angle(self.angle);
            let mov_inc = random_range(Self::BIRD_SPEED_MIN * 0.5, Self::BIRD_SPEED_MAX * 0.5); 
            self.xy.x += mov_inc * self.angle.cos();
            self.xy.y += mov_inc * self.angle.sin();
            
            self.angle += self.v_screen_edge(inner, deg_to_rad(Self::TURN_ANGLE), Self::TURN_GAIN);
        
            self.angle = self.wrap_angle(self.angle);
            let mov_inc = random_range(Self::BIRD_SPEED_MIN * 0.5, Self::BIRD_SPEED_MAX * 0.5); 
            self.xy.x += mov_inc * self.angle.cos();
            self.xy.y += mov_inc * self.angle.sin();
        }



        self.screen_wrap(win);
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
    
    fn wrap_angle_180(&self, angle: f32) -> f32{
        let ref_angle = angle % (2.0 * std::f32::consts::PI);
        let mut wrapped_angle = ref_angle;
        
        if ref_angle <= -std::f32::consts::PI{
            wrapped_angle = ref_angle + ( 2.0 * std::f32::consts::PI );
        }
        else if ref_angle >= std::f32::consts::PI{
            wrapped_angle = ref_angle - ( 2.0 * std::f32::consts::PI ); 
        }
        
        assert!(wrapped_angle >= -180.0);
        assert!(wrapped_angle <= 180.0);
        wrapped_angle
    }

    pub fn spatial_awareness(&mut self, angle: f32, gain: f32, lower_speed: f32, upper_speed: f32, randomise: bool) -> f32{
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
        self.xy.x += mov_inc * angle.cos();
        self.xy.y += mov_inc * angle.sin();
        let delta = (self.xy.y - old_xy.y).atan2(self.xy.x - old_xy.x);
        delta * gain
    }
    
    pub fn spatial_awareness_redux(&mut self, angle: f32, rot_angle: f32, gain: f32, lower_speed: f32, upper_speed: f32, randomise: bool) -> f32{
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

        /* 1. Move bird in direction of angle */
        self.xy.x += mov_inc * angle.cos();
        self.xy.y += mov_inc * angle.sin();

        /* 2. Calculate how much bird should rotate away from the reference_bird */
        let angle_offset = 0.0 - angle;
        
        /* 3. rotate the original point */
        let rotated_position = self.rotate(old_xy, angle_offset);
        let delta:f32;

        /* 4. Determine whether to add or subtract an angle to turn away as appropriate */
        if rotated_position.y.is_positive()
        {
            delta = deg_to_rad(rot_angle);
        }
        else
        {
            delta = deg_to_rad(-rot_angle);
        }

        //let delta = (self.xy.y - old_xy.y).atan2(self.xy.x - old_xy.x);
        delta * gain
    }
    
    pub fn apply_separation(&mut self, angle: f32, rot_angle: f32, gain: f32, lower_speed: f32, upper_speed: f32, randomise: bool){
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

        /* 1. Move bird in direction of angle */
        self.xy.x += mov_inc * angle.cos();
        self.xy.y += mov_inc * angle.sin();

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
        self.angle += delta * gain;
        self.angle = self.wrap_angle(self.angle);
        println!("new_angle: {:?}", rad_to_deg(self.angle));
    }
    
    pub fn apply_cohesion(&mut self, angle: f32, rot_angle: f32, gain: f32, lower_speed: f32, upper_speed: f32, randomise: bool){
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

        /* 1. Move bird in direction of angle */
        self.xy.x += mov_inc * angle.cos();
        self.xy.y += mov_inc * angle.sin();

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
        self.angle += delta * gain;
        self.angle = self.wrap_angle(self.angle);
        println!("new_angle: {:?}", rad_to_deg(self.angle));
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
            let mut angle = self.angle - (std::f32::consts::PI / 2.0);
            angle = self.wrap_angle(angle);
            if rad_to_deg(angle) > 180.0{
                turn = -1.0;
            }
            diff = turn * turn_angle * gain * delta.exp();
        }
        else if self.xy.y < inner.bottom() as f32{
            let mut delta = inner.bottom() - self.xy.y;
            delta *= Self::DECAY;
            let mut angle = self.angle - (std::f32::consts::PI * 1.5);
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
            //let mut angle = self.angle - (std::f32::consts::PI * 1.5);
            let mut angle = self.angle;
            angle = self.wrap_angle(angle);

            if rad_to_deg(angle) > 180.0{
                turn = -1.0;
            }
            diff = turn * turn_angle * gain * delta.exp();
        }
        else if self.xy.x < inner.left() as f32{
            let mut delta = inner.left() - self.xy.x;
            delta *= Self::DECAY;
            let mut angle = self.angle - (std::f32::consts::PI);
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

        let gain = 1.0;
        let lower_speed = 1.0;
        let upper_speed = 1.0;
        let rotation_angle = deg_to_rad(1.0);

        bird.apply_separation(dir_angle, rotation_angle, gain, lower_speed, upper_speed, false);

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
    fn spatial_awareness_east(){
        let x = 0.0;
        let y = 0.0;
        let angle = 0.0;
        let mut bird = Bird::new(pt2(x, y), angle);
        
        assert_eq!(bird.position().x, x);
        assert_eq!(bird.position().y, y);
        assert_eq!(bird.angle(), angle);
        assert_eq!(bird.get_separation(), angle);
        assert_eq!(bird.get_alignment(), 0.0);
        assert_eq!(bird.get_cohesion(), angle);

        let dir_angle = 0.0;
        let gain = 1.0;
        let lower_speed = 1.0;
        let upper_speed = 1.0;
        let angle_fract = bird.spatial_awareness(dir_angle, gain, lower_speed, upper_speed, false);

        assert!(compare_floats(angle_fract, 0.0, FLOAT_PRECISION));
        assert!(compare_floats(bird.position().x, 1.0, FLOAT_PRECISION));
        assert!(compare_floats(bird.position().y, 0.0, FLOAT_PRECISION));
        assert!(compare_floats(bird.angle(), 0.0, FLOAT_PRECISION));
    }
    
    #[test]
    fn spatial_awareness_west(){
        let x = 0.0;
        let y = 0.0;
        let angle = 0.0;
        let mut bird = Bird::new(pt2(x, y), angle);
        
        assert_eq!(bird.position().x, x);
        assert_eq!(bird.position().y, y);
        assert_eq!(bird.angle(), angle);
        assert_eq!(bird.get_separation(), angle);
        assert_eq!(bird.get_alignment(), 0.0);
        assert_eq!(bird.get_cohesion(), angle);

        let dir_angle = std::f32::consts::PI;
        let gain = 1.0;
        let lower_speed = 1.0;
        let upper_speed = 1.0;
        let angle_fract = bird.spatial_awareness(dir_angle, gain, lower_speed, upper_speed, false);


        /* NOTE: abs() used here because angle can be calculated as -180 */
        assert!(compare_floats(angle_fract.abs(), dir_angle, FLOAT_PRECISION));
        assert!(compare_floats(bird.position().x, -1.0, FLOAT_PRECISION));
        assert!(compare_floats(bird.position().y, 0.0, FLOAT_PRECISION));
        assert!(compare_floats(bird.angle(), 0.0, FLOAT_PRECISION));
       
        /* Additional test to assert above :) */
        let mut bird_neg = Bird::new(pt2(x, y), angle);
        let dir_angle_neg = -std::f32::consts::PI;
        let angle_fract = bird_neg.spatial_awareness(dir_angle_neg, gain, lower_speed, upper_speed, false);

        /* NOTE: abs() used here because angle can be calculated as -180 */
        assert!(compare_floats(angle_fract.abs(), dir_angle, FLOAT_PRECISION));
        assert!(compare_floats(bird_neg.position().x, -1.0, FLOAT_PRECISION));
        assert!(compare_floats(bird_neg.position().y, 0.0, FLOAT_PRECISION));
        assert!(compare_floats(bird_neg.angle(), 0.0, FLOAT_PRECISION));
    }
    
    #[test]
    fn spatial_awareness_north(){
        let x = 0.0;
        let y = 0.0;
        let angle = 0.0;
        let mut bird = Bird::new(pt2(x, y), angle);
        
        assert_eq!(bird.position().x, x);
        assert_eq!(bird.position().y, y);
        assert_eq!(bird.angle(), angle);
        assert_eq!(bird.get_separation(), angle);
        assert_eq!(bird.get_alignment(), 0.0);
        assert_eq!(bird.get_cohesion(), angle);

        let dir_angle = std::f32::consts::PI / 2.0;
        let gain = 1.0;
        let lower_speed = 1.0;
        let upper_speed = 1.0;
        let angle_fract = bird.spatial_awareness(dir_angle, gain, lower_speed, upper_speed, false);

        assert!(compare_floats(angle_fract, dir_angle, FLOAT_PRECISION));
        assert!(compare_floats(bird.position().x, 0.0, FLOAT_PRECISION));
        assert!(compare_floats(bird.position().y, 1.0, FLOAT_PRECISION));
        assert!(compare_floats(bird.angle(), 0.0, FLOAT_PRECISION));
    }
    
    #[test]
    fn spatial_awareness_south(){
        let x = 0.0;
        let y = 0.0;
        let angle = 0.0;
        let mut bird = Bird::new(pt2(x, y), angle);
        
        assert_eq!(bird.position().x, x);
        assert_eq!(bird.position().y, y);
        assert_eq!(bird.angle(), angle);
        assert_eq!(bird.get_separation(), angle);
        assert_eq!(bird.get_alignment(), 0.0);
        assert_eq!(bird.get_cohesion(), angle);

        let dir_angle = -(std::f32::consts::PI / 2.0);
        let gain = 1.0;
        let lower_speed = 1.0;
        let upper_speed = 1.0;
        let angle_fract = bird.spatial_awareness(dir_angle, gain, lower_speed, upper_speed, false);

        assert!(compare_floats(angle_fract, dir_angle, FLOAT_PRECISION));
        assert!(compare_floats(bird.position().x, 0.0, FLOAT_PRECISION));
        assert!(compare_floats(bird.position().y, -1.0, FLOAT_PRECISION));
        assert!(compare_floats(bird.angle(), 0.0, FLOAT_PRECISION));
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
}
