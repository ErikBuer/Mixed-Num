use mixed_num::*;

#[test]
fn test_cartesian_external_interface() {
    
    let temp = Cartesian::new(1f32,-1f32);
    assert_eq!{ temp.to_string(), "1-1i" };

    let temp2 = temp*2f32;
    assert_eq!{ temp2.to_string(), "2-2i" };


    let mut c_num = Cartesian::new(0f32,f32::mixed_pi());
    
    c_num = c_num.mixed_exp();
    assert_eq!{ c_num.to_string(), "-1+-0i" };
    
    let mut c_num = Cartesian::new(1f32,f32::mixed_pi());
    
    c_num = c_num.mixed_exp();
    assert_eq!{ c_num.to_string(), "-2.7182817+-0i" };
    
    let mut c_num = Cartesian::new(1f32,0f32);
    
    c_num = c_num.mixed_exp();
    assert_eq!{ c_num.to_string(), "2.7182817+0i" };
    
}