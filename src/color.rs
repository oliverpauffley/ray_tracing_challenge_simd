use std::ops::{Add, Index, Mul, Sub};

use crate::float_equal;

#[derive(Debug, Clone, Copy)]
pub struct Color(std::simd::f64x4);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self(std::simd::f64x4::from_array([r, g, b, 0.0]))
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Color(self.0 * rhs.0)
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        use Colors::*;
        Self::new(self[R] * rhs, self[G] * rhs, self[B] * rhs)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        use Colors::*;

        if !float_equal(self[R], other[R]) {
            false
        } else if !float_equal(self[G], other[G]) {
            false
        } else if !float_equal(self[B], other[B]) {
            false
        } else {
            true
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

#[derive(Debug)]
pub enum Colors {
    R,
    G,
    B,
}

impl Index<Colors> for Color {
    type Output = f64;

    fn index(&self, index: Colors) -> &Self::Output {
        match index {
            Colors::R => &self.0[0],
            Colors::G => &self.0[1],
            Colors::B => &self.0[2],
        }
    }
}

#[cfg(test)]
mod test_color {
    use super::*;

    #[test]
    fn can_create_a_color() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(-0.5, c[Colors::R]);
        assert_eq!(0.4, c[Colors::G]);
        assert_eq!(1.7, c[Colors::B]);
    }

    #[test]
    fn adding_colors() {
        let a = Color::new(0.9, 0.6, 0.75);
        let b = Color::new(0.1, 0.4, 0.25);

        let want = Color::new(1.0, 1.0, 1.0);

        assert_eq!(want, a + b)
    }

    #[test]
    fn subtracting_colors() {
        let a = Color::new(1.0, 1.0, 1.0);
        let b = Color::new(0.9, 0.6, 0.75);

        let want = Color::new(0.1, 0.4, 0.25);

        assert_eq!(want, a - b)
    }

    #[test]
    fn multipling_colors() {
        let a = Color::new(1.0, 2.0, 3.0);
        let b = Color::new(2.0, 0.5, 3.0);

        let want = Color::new(2.0, 1.0, 9.0);

        assert_eq!(want, a * b);
    }

    #[test]
    fn multiplying_color_by_scalar() {
        let a = Color::new(1.0, 2.0, 3.0);

        let want = Color::new(2.0, 4.0, 6.0);

        assert_eq!(want, a * 2.0)
    }
}
