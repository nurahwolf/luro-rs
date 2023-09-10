use core::fmt;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use crate::RollValue;

impl From<RollValue> for f64 {
    fn from(v: RollValue) -> Self {
        match v {
            RollValue::Int(i) => i as f64,
            RollValue::Float(f) => f,
        }
    }
}

impl fmt::Display for RollValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Float(v) => f.write_str(&v.to_string()),
            Self::Int(v) => f.write_str(&v.to_string()),
        }
    }
}

impl Add for RollValue {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Float(i), Self::Float(j)) => Self::Float(i + j),
            (Self::Int(i), Self::Float(j)) => Self::Float(i as f64 + j),
            (Self::Float(i), Self::Int(j)) => Self::Float(i + j as f64),
            (Self::Int(i), Self::Int(j)) => Self::Int(i + j),
        }
    }
}

impl Sub for RollValue {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Float(i), Self::Float(j)) => Self::Float(i - j),
            (Self::Int(i), Self::Float(j)) => Self::Float(i as f64 - j),
            (Self::Float(i), Self::Int(j)) => Self::Float(i - j as f64),
            (Self::Int(i), Self::Int(j)) => Self::Int(i - j),
        }
    }
}

impl Mul for RollValue {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Float(i), Self::Float(j)) => Self::Float(i * j),
            (Self::Int(i), Self::Float(j)) => Self::Float(i as f64 * j),
            (Self::Float(i), Self::Int(j)) => Self::Float(i * j as f64),
            (Self::Int(i), Self::Int(j)) => Self::Int(i * j),
        }
    }
}

impl Div for RollValue {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Float(i), Self::Float(j)) => Self::Float(i / j),
            (Self::Int(i), Self::Float(j)) => Self::Float(i as f64 / j),
            (Self::Float(i), Self::Int(j)) => Self::Float(i / j as f64),
            (Self::Int(i), Self::Int(j)) => Self::Float(i as f64 / j as f64),
        }
    }
}

impl Rem for RollValue {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Float(i), Self::Float(j)) => Self::Float(i % j),
            (Self::Int(i), Self::Float(j)) => Self::Float(i as f64 % j),
            (Self::Float(i), Self::Int(j)) => Self::Float(i % j as f64),
            (Self::Int(i), Self::Int(j)) => Self::Int(i % j),
        }
    }
}

impl Neg for RollValue {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Float(i) => Self::Float(-i),
            Self::Int(i) => Self::Int(-i),
        }
    }
}

impl RollValue {
    pub fn floor(self) -> Self {
        match self {
            Self::Float(i) => Self::Int(i.floor() as i64),
            i => i,
        }
    }

    pub fn pow(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Float(i), Self::Float(j)) => Self::Float(i.powf(j)),
            (Self::Int(i), Self::Float(j)) => Self::Float((i as f64).powf(j)),
            (Self::Float(i), Self::Int(j)) => Self::Float(i.powf(j as f64)),
            (Self::Int(i), Self::Int(j)) if j < 0 => Self::Float((i as f64).powf(j as f64)),
            (Self::Int(i), Self::Int(j)) => Self::Int(i.pow(j as u32)),
        }
    }
}
