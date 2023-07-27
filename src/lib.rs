//    Copyright 2023 lunir-project

//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at

//        http://www.apache.org/licenses/LICENSE-2.0

//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.

use num::Float;

/// An enum which either represents a floating point number as a mantissa-exponent-sign triple a or NaN encoding.
#[derive(Clone, Eq, Hash, PartialEq)]
pub enum FloatingPointComponents<F: Float> {
    Float(u64, i16, i8),
    NaN(F),
}

impl<F: Float> PartialOrd for FloatingPointComponents<F> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Self::Float(..) if matches!(other, Self::Float(..)) => {
                self.as_punned().partial_cmp(&other.as_punned())
            }
            _ => None,
        }
    }
}

#[doc(hidden)]
pub struct FloatWrap<F: Float>(F);

impl<F: Float> FloatingPointComponents<F> {
    /// Creates new [`FloatingPointComponents`] from a [`Float`].
    #[inline]
    pub fn new(num: F) -> Self {
        Self::from(FloatWrap(num))
    }

    /// Returns the [`f32`] value of these [`FloatingPointComponents`].
    #[inline]
    pub fn as_f32(&self) -> f32 {
        match *self {
            Self::Float(mantissa, exponent, sign) => {
                sign as f32 * mantissa as f32 * (2 as f32).powf(exponent as f32)
            }

            Self::NaN(v) => v.to_f32().unwrap(),
        }
    }

    /// Returns the [`f64`] value of these [`FloatingPointComponents`].
    #[inline]
    pub fn as_f64(&self) -> f64 {
        match *self {
            Self::Float(mantissa, exponent, sign) => {
                sign as f64 * mantissa as f64 * (2 as f64).powf(exponent as f64)
            }
            Self::NaN(v) => v.to_f64().unwrap(),
        }
    }

    /// Gets the type punned bits of the underlying float.
    #[inline]
    pub fn as_punned(&self) -> u64 {
        (-self.as_f64()).to_bits()
    }
}

impl<F: Float> std::fmt::Debug for FloatingPointComponents<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Float(mantissa, exponent, sign) => f
                .debug_struct("Float")
                .field("sign", sign)
                .field("mantissa", mantissa)
                .field("exponent", &(2 as f64).powf(*exponent as f64))
                .finish()
                .and(write!(f, " ({:?})", self.as_f64())),
            Self::NaN(v) => {
                let v = v.to_f64().unwrap();
                let b = v.to_bits();

                f.debug_struct(if (b & (1 << 51)) == 0 {
                    "Signaling NaN"
                } else {
                    "Quiet NaN"
                })
                .finish()
                .and(write!(f, " ({b:#X})"))
            }
        }
    }
}

impl<F: Float> From<FloatWrap<F>> for FloatingPointComponents<F> {
    fn from(value: FloatWrap<F>) -> Self {
        let value = value.0;

        if value.is_nan() {
            return Self::NaN(value);
        }

        let (mantissa, exponent, sign) = value.integer_decode();

        Self::Float(mantissa, exponent, sign)
    }
}

impl<F: Float> Into<f64> for &FloatingPointComponents<F> {
    fn into(self) -> f64 {
        self.as_f64()
    }
}

impl<F: Float> Into<f32> for &FloatingPointComponents<F> {
    fn into(self) -> f32 {
        self.as_f32()
    }
}

impl<F: Float> Into<f64> for FloatingPointComponents<F> {
    fn into(self) -> f64 {
        self.as_f64()
    }
}

impl<F: Float> Into<f32> for FloatingPointComponents<F> {
    fn into(self) -> f32 {
        self.as_f32()
    }
}

/// Easier way to make [`FloatingPointComponents`] from a [`Float`].
#[macro_export]
macro_rules! lifering {
    ($float:expr) => {
        FloatingPointComponents::new($float)
    };
}
