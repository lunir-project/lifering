use lifering::{lifering, FloatingPointComponents};

#[test]
fn recover_f64() {
    const PI: f64 = 3.141592653589793;
    const EULER: f64 = 2.718281828459045;

    assert_eq!(lifering!(PI).as_f64(), PI);
    assert_eq!(lifering!(EULER).as_f64(), EULER);
}

#[test]
fn recover_f32() {
    const PI: f32 = 3.14159265;
    const EULER: f32 = 2.71828185;

    assert_eq!(lifering!(PI).as_f32(), PI);
    assert_eq!(lifering!(EULER).as_f32(), EULER);
}

#[test]
#[should_panic]
fn compare_nan_f32() {
    lifering!(f32::NAN).partial_cmp(&lifering!(0.0)).unwrap();
}

#[test]
#[should_panic]
fn compare_nan_f64() {
    lifering!(f64::NAN).partial_cmp(&lifering!(0.0)).unwrap();
}

#[test]
fn compare_infinity_f64() {
    assert!(lifering!(f64::INFINITY)
        .partial_cmp(&lifering!(0.0))
        .unwrap()
        .is_gt());
}

#[test]
fn compare_infinity_f32() {
    assert!(lifering!(f32::INFINITY)
        .partial_cmp(&lifering!(0.0))
        .unwrap()
        .is_gt());
}

#[test]
fn compare_neg_infinity_f64() {
    assert!(lifering!(f64::NEG_INFINITY)
        .partial_cmp(&lifering!(0.0))
        .unwrap()
        .is_lt());
}
#[test]
fn compare_neg_infinity_f32() {
    assert!(lifering!(f32::NEG_INFINITY)
        .partial_cmp(&lifering!(0.0))
        .unwrap()
        .is_lt());
}

#[test]
fn widen_f32_nan_to_f64() {
    // lifering conversion from f32 NaN to f64 NaN, comparing punned values because NaN comparison is always false.
    assert_eq!(
        lifering!(lifering!(f32::NAN).as_f64()).as_punned(),
        lifering!(f64::NAN).as_punned()
    );
}

#[test]
fn narrow_f64_nan_to_f32() {
    assert_eq!(
        lifering!(lifering!(f64::NAN).as_f32()).as_punned(),
        lifering!(f32::NAN).as_punned()
    );
}

#[test]
fn narrow_f64_to_f32() {
    assert_eq!(lifering!(f64::INFINITY).as_f32(), f32::INFINITY);
}

#[test]
fn compare_f32_and_f64() {
    assert!(lifering!(10.0)
        .partial_cmp(&lifering!(10.0f32))
        .unwrap()
        .is_eq());
}

#[test]
fn eq_f32_nan() {
    assert!(lifering!(f32::NAN) == lifering!(f32::NAN));
}

#[test]
fn eq_f64_nan() {
    assert!(lifering!(f64::NAN) == lifering!(f64::NAN));
}

#[test]
fn neq_f32_nan() {
    assert!(lifering!(f32::NAN) != lifering!(0.0));
}

#[test]
fn neq_f64_nan() {
    assert!(lifering!(f64::NAN) != lifering!(0.0));
}

#[test]
fn f64_hashmap() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(lifering!(1.0_f64), 1);
    map.insert(lifering!(f64::NAN), 2);

    for (key, value) in &map {
        let key = key.as_f64();
        if key == 1.0 {
            assert_eq!(value, &1);
        } else if key.is_nan() {
            assert_eq!(value, &2);
        } else {
            unreachable!();
        }
    }
}

#[test]
fn f32_hashmap() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(lifering!(1.0_f32), 1);
    map.insert(lifering!(f32::NAN), 2);

    for (key, value) in &map {
        let key = key.as_f32();
        if key == 1.0 {
            assert_eq!(value, &1);
        } else if key.is_nan() {
            assert_eq!(value, &2);
        } else {
            unreachable!();
        }
    }
}
