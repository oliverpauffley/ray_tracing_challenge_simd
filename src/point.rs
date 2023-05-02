use std::{
    ops::{Add, Div, Mul, Neg, Sub},
    simd,
};

use crate::vector::Vector;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Point(pub std::simd::f64x4);

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let vec_vals = [x, y, z, 1.0];
        Self(std::simd::f64x4::from_array(vec_vals))
    }

    pub fn from_array(vals: [f64; 3]) -> Self {
        let vec_vals = [vals[0], vals[1], vals[2], 1.0];
        Self(std::simd::f64x4::from_array(vec_vals))
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector(self.0 - rhs.0)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(std::simd::f64x4::from_array([
            self.0[0] * rhs,
            self.0[1] * rhs,
            self.0[2] * rhs,
            self.0[3],
        ]))
    }
}

impl Div<f64> for Point {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(std::simd::f64x4::from_array([
            self.0[0] / rhs,
            self.0[1] / rhs,
            self.0[2] / rhs,
            self.0[3],
        ]))
    }
}

#[cfg(test)]
mod test_point {
    use super::*;

    #[test]
    fn two_points_equal() {
        let a = Point::new(1.0, 1.0, 1.0);
        let b = Point::new(1.0, 1.0, 1.0);
        assert_eq!(a, b)
    }

    #[test]
    fn two_points_not_equal() {
        let a = Point::new(1.0, -1.0, 1.0);
        let b = Point::new(1.0, 1.0, 1.0);
        assert_ne!(a, b)
    }

    #[test]
    fn add_a_vector_to_a_point() {
        let point = Point::new(3.0, -2.0, 5.0);
        let vector = Vector::new(-2.0, 3.0, 1.0);

        let want = Point::new(1.0, 1.0, 6.0);

        assert_eq!(want, point + vector);
    }

    #[test]
    fn subtract_two_points() {
        let p_1 = Point::new(3.0, 2.0, 1.0);
        let p_2 = Point::new(5.0, 6.0, 7.0);

        let want = Vector::new(-2.0, -4.0, -6.0);

        assert_eq!(want, p_1 - p_2);
    }

    #[test]
    fn subtract_vector_from_point() {
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);

        let want = Point::new(-2.0, -4.0, -6.0);

        assert_eq!(want, p - v);
    }

    #[test]
    fn multiply_point_scalar() {
        let p = Point::new(1.0, 2.0, 3.0);
        let scalar = 3.0;

        let want = Point::new(3.0, 6.0, 9.0);

        assert_eq!(want, p * scalar);
    }
}
