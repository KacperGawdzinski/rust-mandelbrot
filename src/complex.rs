use std::ops::Mul;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub r: f64,
    pub i: f64,
}

impl Complex {
    pub fn zero() -> Self {
        Self { r: 0.0, i: 0.0 }
    }

    pub fn length(&self) -> f64 {
        (self.r * self.r + self.i * self.i).sqrt()
    }

    pub fn length_no_sqrt(&self) -> f64 {
        self.r * self.r + self.i * self.i
    }
}

impl PartialEq<Complex> for Complex {
    fn eq(&self, other: &Complex) -> bool {
        self.r == other.r && self.i == other.i
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;
    fn add(self, other: Complex) -> Self::Output {
        Complex {
            r: self.r + other.r,
            i: self.i + other.i,
        }
    }
}

impl Sub<Complex> for Complex {
    type Output = Complex;
    fn sub(self, other: Complex) -> Self::Output {
        Complex {
            r: self.r - other.r,
            i: self.i - other.i,
        }
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;
    fn mul(self, other: Complex) -> Self::Output {
        Complex {
            r: self.r * other.r - self.i * other.i,
            i: self.i * other.r + self.r * other.i,
        }
    }
}

impl Mul<f64> for Complex {
    type Output = Complex;
    fn mul(self, real: f64) -> Self::Output {
        Complex {
            r: self.r * real,
            i: self.i * real,
        }
    }
}
