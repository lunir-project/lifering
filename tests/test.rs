use lifering::FloatingPointComponents;

#[test]
#[should_panic]
fn test_f64_nan() {
    FloatingPointComponents::new(f64::NAN).unwrap();
}

#[test]
#[should_panic]
fn test_f32_nan() {
    FloatingPointComponents::new(f32::NAN).unwrap();
}

#[test]
fn test_value_recovery_f64() {
    const PI: f64 = 3.141592653589793;
    const EULER: f64 = 2.718281828459045;

    assert_eq!(FloatingPointComponents::new(PI).unwrap().as_f64(), PI);
    assert_eq!(FloatingPointComponents::new(EULER).unwrap().as_f64(), EULER);
}

#[test]
fn test_value_recovery_f32() {
    const PI: f32 = 3.14159265;
    const EULER: f32 = 2.71828185;

    assert_eq!(FloatingPointComponents::new(PI).unwrap().as_f32(), PI);
    assert_eq!(FloatingPointComponents::new(EULER).unwrap().as_f32(), EULER);
}
