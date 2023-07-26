use lifering::{lifering, FloatingPointComponents};

#[test]
fn test_f64_nan() {
    FloatingPointComponents::new(f64::NAN).as_f64();
}

#[test]
fn test_f32_nan() {
    FloatingPointComponents::new(f32::NAN).as_f32();
}

#[test]
fn test_value_recovery_f64() {
    const PI: f64 = 3.141592653589793;
    const EULER: f64 = 2.718281828459045;

    assert_eq!(lifering!(PI).as_f64(), PI);
    assert_eq!(lifering!(EULER).as_f64(), EULER);
}

#[test]
fn test_value_recovery_f32() {
    const PI: f32 = 3.14159265;
    const EULER: f32 = 2.71828185;

    assert_eq!(lifering!(PI).as_f32(), PI);
    assert_eq!(lifering!(EULER).as_f32(), EULER);
}

#[test]
#[should_panic]
fn test_nan_partialord_f32() {
    lifering!(f32::NAN).partial_cmp(&lifering!(0.0)).unwrap();
}

#[test]
#[should_panic]
fn test_nan_partialord_f64() {
    lifering!(f64::NAN).partial_cmp(&lifering!(0.0)).unwrap();
}

#[test]
fn test_infinity_partialord_f64() {
    assert!(lifering!(f64::INFINITY)
        .partial_cmp(&lifering!(0.0))
        .unwrap()
        .is_gt());
}

#[test]
fn test_infinity_partialord_f32() {
    assert!(lifering!(f32::INFINITY)
        .partial_cmp(&lifering!(0.0))
        .unwrap()
        .is_gt());
}

#[test]
fn test_neg_infinity_partialord_f64() {
    assert!(lifering!(f64::NEG_INFINITY)
        .partial_cmp(&lifering!(0.0))
        .unwrap()
        .is_lt());
}
#[test]
fn test_neg_infinity_partialord_f32() {
    assert!(lifering!(f32::NEG_INFINITY)
        .partial_cmp(&lifering!(0.0))
        .unwrap()
        .is_lt());
}

#[test]
fn test_f32_nan_to_f64() {
    // lifering conversion from f32 NaN to f64 NaN, comparing punned values because NaN comparison is always false.
    assert_eq!(
        lifering!(lifering!(f32::NAN).as_f64()).as_punned(),
        lifering!(f64::NAN).as_punned()
    );
}

#[test]
fn test_narrowing_f64_nan_to_f32() {
    assert_eq!(
        lifering!(lifering!(f64::NAN).as_f32()).as_punned(),
        lifering!(f32::NAN).as_punned()
    );
}

#[test]
fn test_narrowing_f64_to_f32() {
    assert_eq!(lifering!(f64::INFINITY).as_f32(), f32::INFINITY);
}
