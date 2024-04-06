pub fn wrap(angle: f32) -> f32{
    let mut ref_angle = angle.rem_euclid(2.0 * std::f32::consts::PI);

    if ref_angle.is_sign_negative()
    {
        if ref_angle >= -std::f32::EPSILON
        {
            ref_angle = 0.0;
        }
    }

    let mut wrapped_angle = ref_angle;
    if ref_angle < 0.0{
        wrapped_angle = ref_angle + ( 2.0 * std::f32::consts::PI );
    }
    else if ref_angle >= ( 2.0 * std::f32::consts::PI ){
        wrapped_angle = ref_angle - ( 2.0 * std::f32::consts::PI ); 
    }
    
    assert!(wrapped_angle >= 0.0);
    assert!(wrapped_angle <  (2.0 * std::f32::consts::PI),"wrapped angle:{:?}, ref: {:?}", wrapped_angle, ref_angle);
    wrapped_angle
}

pub fn wrap_180(angle: f32) -> f32{
    let ref_angle = angle.rem_euclid(2.0 * std::f32::consts::PI);
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
        assert_eq!(-2.0, wrap_180(-2.0));
        
        assert_eq!(std::f32::consts::PI, wrap_180(std::f32::consts::PI));
        assert_eq!(std::f32::consts::PI / 2.0, wrap_180(std::f32::consts::PI / 2.0));
        assert_eq!(std::f32::consts::PI, wrap_180(-std::f32::consts::PI));
        

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

    #[test]
    fn calc_angle_wrap(){
        assert_eq!(0.0, wrap(0.0));
        assert_eq!(2.0, wrap(2.0));
        assert_eq!(std::f32::consts::PI, wrap(std::f32::consts::PI));
        assert_eq!(std::f32::consts::PI, wrap(-std::f32::consts::PI));
        assert_eq!(0.0, wrap(2.0*std::f32::consts::PI));
        assert_eq!(0.0, wrap(-2.0*std::f32::consts::PI));
        assert_eq!(0.0, wrap(4.0*std::f32::consts::PI));
        assert_eq!(0.0, wrap(-4.0*std::f32::consts::PI));

        assert!(cmp_floats(deg_to_rad(270.0), wrap(deg_to_rad(630.0)),FLOAT_PRECISION));
        assert!(cmp_floats(deg_to_rad(90.0), wrap(deg_to_rad(-630.0)),FLOAT_PRECISION));
        
        assert!(cmp_floats(0.0, wrap(-5.7742e-8),FLOAT_PRECISION));
        assert!(cmp_floats(0.0, wrap(5.7742e-8),FLOAT_PRECISION));
        assert!(cmp_floats(0.0, wrap(std::f32::EPSILON),FLOAT_PRECISION));
        assert!(cmp_floats(0.0, wrap(-std::f32::EPSILON),FLOAT_PRECISION));
        assert!(cmp_floats(0.0, wrap(0.0000000000001),FLOAT_PRECISION));
        assert!(cmp_floats(0.0, wrap(-0.000000000001),FLOAT_PRECISION));
        assert!(cmp_floats(0.0, wrap(-0.0),FLOAT_PRECISION));
        assert!(cmp_floats(0.0, wrap(-std::f32::EPSILON),FLOAT_PRECISION));
        assert!(cmp_floats(0.0, wrap(std::f32::EPSILON),FLOAT_PRECISION));
        assert!(cmp_floats(0.0, wrap(-std::f32::EPSILON + (-2.0 * std::f32::consts::PI)),FLOAT_PRECISION));
        assert!(cmp_floats(0.0, wrap(std::f32::EPSILON + (-2.0 * std::f32::consts::PI)),FLOAT_PRECISION));
        assert!(cmp_floats(0.0, wrap(-std::f32::EPSILON + (2.0 * std::f32::consts::PI)),FLOAT_PRECISION));
        assert!(cmp_floats(0.0, wrap(std::f32::EPSILON + (2.0 * std::f32::consts::PI)),FLOAT_PRECISION));
        
        assert!(cmp_floats(0.0, wrap(-0.00000000001 + (-2.0 * std::f32::consts::PI)),FLOAT_PRECISION));
        assert!(cmp_floats(0.0, wrap( 0.00000000001 + (-2.0 * std::f32::consts::PI)),FLOAT_PRECISION));
        assert!(cmp_floats(0.0, wrap(-0.00000000001 + (2.0 * std::f32::consts::PI)),FLOAT_PRECISION));
        assert!(cmp_floats(0.0, wrap( 0.00000000001 + (2.0 * std::f32::consts::PI)),FLOAT_PRECISION));
    }

    #[test]
    fn test_euclid(){
       
        let wrap:f32 = 10.0;
        
        let a:f32 = 1.0;
        assert_eq!(1.0, a.rem_euclid(wrap));
        
        let b:f32 = 10.0;
        assert_eq!(0.0, b.rem_euclid(wrap));
        
        let c:f32 = -1.0;
        assert_eq!(9.0, c.rem_euclid(wrap));
        
        let d:f32 = 0.0;
        assert_eq!(0.0, d.rem_euclid(wrap));
    }
}
