use std::ops::{Add, Neg, Sub};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Vector(pub std::simd::f64x4);

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let vec_vals = [x, y, z, 0.0];
        Self(std::simd::f64x4::from_array(vec_vals))
    }

    pub fn from_array(vals: [f64; 3]) -> Self {
        let vec_vals = [vals[0], vals[1], vals[2], 0.0];
        Self(std::simd::f64x4::from_array(vec_vals))
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector(self.0 + rhs.0)
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector(self.0 - rhs.0)
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector(std::simd::f64x4::splat(0.0) - self.0)
    }
}

#[cfg(test)]
mod test_vector {
    use super::*;

    #[test]
    fn two_vectors_equal() {
        let a = Vector::new(1.0, 1.0, 1.0);
        let b = Vector::new(1.0, 1.0, 1.0);
        assert_eq!(a, b)
    }

    #[test]
    fn two_vectors_not_equal() {
        let a = Vector::new(1.0, -1.0, 1.0);
        let b = Vector::new(1.0, 1.0, 1.0);
        assert_ne!(a, b)
    }

    #[test]
    fn adding_two_vectors() {
        let a = Vector::new(1.0, -1.0, 1.0);
        let b = Vector::new(1.0, 1.0, 1.0);

        let want = Vector::new(2.0, 0.0, 2.0);

        assert_eq!(want, a + b);
    }
    #[test]
    fn subtracing_two_vectors() {
        let a = Vector::new(1.0, -1.0, 1.0);
        let b = Vector::new(1.0, 1.0, 1.0);

        let want = Vector::new(0.0, -2.0, 0.0);

        assert_eq!(want, a - b);
    }

    #[test]
    fn negate_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);

        let want = Vector::new(-1.0, -2.0, -3.0);

        assert_eq!(want, -v);
    }
}
