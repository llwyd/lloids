pub fn wrap(angle: f32) -> f32{
    let ref_angle = angle % (2.0 * std::f32::consts::PI);
    let mut wrapped_angle = ref_angle;
    
    if ref_angle < 0.0{
        wrapped_angle = ref_angle + ( 2.0 * std::f32::consts::PI );
    }
    else if ref_angle >= ( 2.0 * std::f32::consts::PI ){
        wrapped_angle = ref_angle - ( 2.0 * std::f32::consts::PI ); 
    }
    
    assert!(wrapped_angle >= 0.0);
    assert!(wrapped_angle < (2.0 * std::f32::consts::PI) );
    wrapped_angle
}

pub fn wrap_180(angle: f32) -> f32{
    let ref_angle = angle % (2.0 * std::f32::consts::PI);
    let mut wrapped_angle = ref_angle;
    
    if ref_angle < -std::f32::consts::PI{
        wrapped_angle = ref_angle + ( 2.0 * std::f32::consts::PI );
    }
    else if ref_angle > std::f32::consts::PI{
        wrapped_angle = ref_angle - ( 2.0 * std::f32::consts::PI ); 
    }
    
    assert!(wrapped_angle >= -std::f32::consts::PI);
    assert!(wrapped_angle <= std::f32::consts::PI);
    wrapped_angle
}

#[cfg(test)]
mod tests {
    use nannou::prelude::*;
    use super::*;    
    const FLOAT_PRECISION:f32 = 0.00001;
   
    fn cmp_floats(x:f32, y:f32, precision:f32)->bool{

        let delta = (x - y).abs();
        delta <= precision
    }

    #[test]
    fn calc_angle_wrap_180(){
        assert_eq!(0.0, wrap_180(0.0));
        assert_eq!(1.0, wrap_180(1.0));
        assert_eq!(-1.0, wrap_180(-1.0));
        
        assert_eq!(std::f32::consts::PI, wrap_180(std::f32::consts::PI));
        assert_eq!(-std::f32::consts::PI, wrap_180(-std::f32::consts::PI));
        

        assert_eq!(0.0, wrap_180(2.0*std::f32::consts::PI));
        assert_eq!(0.0, wrap_180(-2.0*std::f32::consts::PI));
        
        assert!(cmp_floats(deg_to_rad(-90.0), wrap_180(deg_to_rad(270.0)), FLOAT_PRECISION));
        assert!(cmp_floats(deg_to_rad(90.0), wrap_180(deg_to_rad(-270.0)), FLOAT_PRECISION));
        
        assert!(cmp_floats(deg_to_rad(-135.0), wrap_180(deg_to_rad(225.0)), FLOAT_PRECISION));
        assert!(cmp_floats(deg_to_rad(135.0), wrap_180(deg_to_rad(-225.0)), FLOAT_PRECISION));
        
        assert!(cmp_floats(deg_to_rad(-179.0), wrap_180(deg_to_rad(181.0)), FLOAT_PRECISION));
        assert!(cmp_floats(deg_to_rad(179.0), wrap_180(deg_to_rad(-181.0)), FLOAT_PRECISION));
        
        assert!(cmp_floats(deg_to_rad(179.0), wrap_180(deg_to_rad(179.0)), FLOAT_PRECISION));
        assert!(cmp_floats(deg_to_rad(-179.0), wrap_180(deg_to_rad(-179.0)), FLOAT_PRECISION));
    }

}
