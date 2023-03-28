use num::Float;
use std::fmt::Debug;

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct FloatingPointScalars(u64, i16, i8);

/// Error type that indicates a `NaN` was used to attempt to create a [`FloatingPointScalars`].
#[derive(Debug)]
pub struct NanError;

/// A wrapper struct around floats to avoid conflict with blanket impl of TryFrom.
pub struct FloatWrap<F: Float>(F);

impl FloatingPointScalars {
    /// Creates a new [`FloatingPointScalars`] from a [`Float`].
    #[inline]
    pub fn new<F: Float>(num: F) -> Result<Self, NanError> {
        Self::try_from(FloatWrap(num))
    }
}

impl FloatingPointScalars {
    #[inline]
    pub fn as_f64(&self) -> f64 {
        let sign_f = self.2 as f64;
        let mantissa_f = self.0 as f64;
        let exponent_f = (2 as f64).powf(self.1 as f64);

        sign_f * mantissa_f * exponent_f
    }

    #[inline]
    pub fn as_f32(&self) -> f32 {
        let sign_f = self.2 as f32;
        let mantissa_f = self.0 as f32;
        let exponent_f = (2 as f32).powf(self.1 as f32);

        sign_f * mantissa_f * exponent_f
    }
}

impl std::fmt::Debug for FloatingPointScalars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Float")
            .field("sign", &self.2)
            .field("mantissa", &self.0)
            .field("exponent", &(2 as f64).powf(self.1 as f64))
            .finish()
            .and(write!(f, " ({})", self.as_f64()))
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
