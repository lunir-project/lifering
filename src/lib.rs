use num::Float;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FloatingPointScalars(u64, i16, i8);

#[derive(Debug)]
pub struct NanError;

pub struct FloatWrap<F: Float>(F);

impl FloatingPointScalars {
    #[inline]
    pub fn new<F: Float>(num: F) -> Result<Self, NanError> {
        Self::try_from(FloatWrap(num))
    }
}

impl<F: Float> TryFrom<FloatWrap<F>> for FloatingPointScalars {
    type Error = NanError;

    fn try_from(value: FloatWrap<F>) -> Result<Self, Self::Error> {
        let value = value.0;

        if value.is_nan() {
            return Err(NanError);
        }

        let (mantissa, exponent, sign) = value.integer_decode();

        Ok(Self(mantissa, exponent, sign))
    }
}

impl Into<f64> for &FloatingPointScalars {
    fn into(self) -> f64 {
        let sign_f = self.2 as f64;
        let mantissa_f = self.0 as f64;
        let exponent_f = (2 as f64).powf(self.1 as f64);

        sign_f * mantissa_f * exponent_f
    }
}

impl Into<f32> for &FloatingPointScalars {
    fn into(self) -> f32 {
        let sign_f = self.2 as f32;
        let mantissa_f = self.0 as f32;
        let exponent_f = (2 as f32).powf(self.1 as f32);

        sign_f * mantissa_f * exponent_f
    }
}

impl Into<f64> for FloatingPointScalars {
    fn into(self) -> f64 {
        let sign_f = self.2 as f64;
        let mantissa_f = self.0 as f64;
        let exponent_f = (2 as f64).powf(self.1 as f64);

        sign_f * mantissa_f * exponent_f
    }
}

impl Into<f32> for FloatingPointScalars {
    fn into(self) -> f32 {
        let sign_f = self.2 as f32;
        let mantissa_f = self.0 as f32;
        let exponent_f = (2 as f32).powf(self.1 as f32);

        sign_f * mantissa_f * exponent_f
    }
}
