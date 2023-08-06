use core::fmt;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use super::RollValue;

impl From<RollValue> for f64 {
    fn from(v: RollValue) -> Self {
        match v {
            RollValue::Int(i) => i as f64,
            RollValue::Float(f) => f
        }
    }
}

impl fmt::Display for RollValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Float(v) => f.write_str(&v.to_string()),
            Self::Int(v) => f.write_str(&v.to_string())
        }
    }
}

impl Add for RollValue {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (RollValue::Float(i), RollValue::Float(j)) => RollValue::Float(i + j),
            (RollValue::Int(i), RollValue::Float(j)) => RollValue::Float(i as f64 + j),
            (RollValue::Float(i), RollValue::Int(j)) => RollValue::Float(i + j as f64),
            (RollValue::Int(i), RollValue::Int(j)) => RollValue::Int(i + j)
        }
    }
}

impl Sub for RollValue {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (RollValue::Float(i), RollValue::Float(j)) => RollValue::Float(i - j),
            (RollValue::Int(i), RollValue::Float(j)) => RollValue::Float(i as f64 - j),
            (RollValue::Float(i), RollValue::Int(j)) => RollValue::Float(i - j as f64),
            (RollValue::Int(i), RollValue::Int(j)) => RollValue::Int(i - j)
        }
    }
}

impl Mul for RollValue {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (RollValue::Float(i), RollValue::Float(j)) => RollValue::Float(i * j),
            (RollValue::Int(i), RollValue::Float(j)) => RollValue::Float(i as f64 * j),
            (RollValue::Float(i), RollValue::Int(j)) => RollValue::Float(i * j as f64),
            (RollValue::Int(i), RollValue::Int(j)) => RollValue::Int(i * j)
        }
    }
}

impl Div for RollValue {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (RollValue::Float(i), RollValue::Float(j)) => RollValue::Float(i / j),
            (RollValue::Int(i), RollValue::Float(j)) => RollValue::Float(i as f64 / j),
            (RollValue::Float(i), RollValue::Int(j)) => RollValue::Float(i / j as f64),
            (RollValue::Int(i), RollValue::Int(j)) => RollValue::Float(i as f64 / j as f64)
        }
    }
}

impl Rem for RollValue {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (RollValue::Float(i), RollValue::Float(j)) => RollValue::Float(i % j),
            (RollValue::Int(i), RollValue::Float(j)) => RollValue::Float(i as f64 % j),
            (RollValue::Float(i), RollValue::Int(j)) => RollValue::Float(i % j as f64),
            (RollValue::Int(i), RollValue::Int(j)) => RollValue::Int(i % j)
        }
    }
}

impl Neg for RollValue {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            RollValue::Float(i) => RollValue::Float(-i),
            RollValue::Int(i) => RollValue::Int(-i)
        }
    }
}

impl RollValue {
    pub fn floor(self) -> Self {
        match self {
            RollValue::Float(i) => RollValue::Int(i.floor() as i64),
            i => i
        }
    }

    pub fn pow(self, rhs: Self) -> Self {
        match (self, rhs) {
            (RollValue::Float(i), RollValue::Float(j)) => RollValue::Float(i.powf(j)),
            (RollValue::Int(i), RollValue::Float(j)) => RollValue::Float((i as f64).powf(j)),
            (RollValue::Float(i), RollValue::Int(j)) => RollValue::Float(i.powf(j as f64)),
            (RollValue::Int(i), RollValue::Int(j)) if j < 0 => RollValue::Float((i as f64).powf(j as f64)),
            (RollValue::Int(i), RollValue::Int(j)) => RollValue::Int(i.pow(j as u32))
        }
    }
}
