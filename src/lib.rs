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

use std::hash::Hash;

use num::Float;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[doc(hidden)]
pub struct FloatEncoding(u64, i16, i8);

/// An enum which either represents a floating point number as a mantissa-exponent-sign triple a or NaN encoding.
#[derive(Clone)]
pub enum FloatingPointComponents<F: Float> {
    Float(FloatEncoding),
    NaN(F),
}

impl<F: Float> Eq for FloatingPointComponents<F> {}

impl<F: Float> Hash for FloatingPointComponents<F> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Float(encoding) => encoding.hash(state),
            Self::NaN(v) => v.to_f64().unwrap().to_bits().hash(state),
        }
    }
}

impl<F: Float> PartialEq for FloatingPointComponents<F> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Float(..) => self.as_f64() == other.as_f64(),
            Self::NaN(_) if matches!(other, Self::NaN(_)) => self.as_punned() == other.as_punned(),
            _ => false,
        }
    }
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
#[derive(Clone, Copy, Eq)]
pub struct FloatWrap<F: Float>(F);

impl<F: Float> PartialEq for FloatWrap<F> {
    fn eq(&self, other: &Self) -> bool {
        if self.0.is_nan() {
            self.0.to_f64().unwrap().to_bits() == other.0.to_f64().unwrap().to_bits()
        } else {
            self.0 == other.0
        }
    }
}

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
            Self::Float(encoding) => {
                let FloatEncoding(mantissa, exponent, sign) = encoding;
                sign as f32 * mantissa as f32 * (2 as f32).powf(exponent as f32)
            }

            Self::NaN(v) => v.to_f32().unwrap(),
        }
    }

    /// Returns the [`f64`] value of these [`FloatingPointComponents`].
    #[inline]
    pub fn as_f64(&self) -> f64 {
        match *self {
            Self::Float(encoding) => {
                let FloatEncoding(mantissa, exponent, sign) = encoding;
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
            Self::Float(encoding) => {
                let FloatEncoding(mantissa, exponent, sign) = encoding;
                f.debug_struct("Float")
                    .field("sign", sign)
                    .field("mantissa", mantissa)
                    .field("exponent", &(2 as f64).powf(*exponent as f64))
                    .finish()
                    .and(write!(f, " ({:?})", self.as_f64()))
            }
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

        Self::Float(FloatEncoding(mantissa, exponent, sign))
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
