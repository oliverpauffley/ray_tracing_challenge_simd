use std::ops::{Add, Div, Index, Mul, Neg, Sub};

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

    /// Calculate the magnitude of a vector.
    pub fn mag(&self) -> f64 {
        f64::sqrt(self.0.as_array().iter().map(|x| x * x).sum())
    }

    /// Normalize a vector by scaling by it's magnitude.
    pub fn norm(self) -> Vector {
        self / self.mag()
    }
}

#[derive(Debug)]
pub enum Direction {
    X,
    Y,
    Z,
}

impl Index<Direction> for Vector {
    type Output = f64;

    fn index(&self, index: Direction) -> &Self::Output {
        match index {
            Direction::X => &self.0[0],
            Direction::Y => &self.0[1],
            Direction::Z => &self.0[2],
        }
    }
}

/// Calculate the vector dot product.
/// ```
///
/// let a = Vector::new(1.0, 2.0, 3.0);
/// let b = Vector::new(2.0, 3.0, 4.0);

/// assert_eq!(20.0, dot(a, b))
pub fn dot(a: Vector, b: Vector) -> f64 {
    use Direction::*;
    a[X] * b[X] + a[Y] * b[Y] + a[Z] * b[Z]
}

/// Calculate the vector cross product.
/// Finds the perpendicular vector to two vectors
/// let a = Vector::new(1.0, 2.0, 3.0);
/// let b = Vector::new(2.0, 3.0, 4.0);

/// let a_b = Vector::new(-1.0, 2.0, -1.0);
/// let b_a = Vector::new(1.0, -2.0, 1.0);

/// assert_eq!(a_b, cross(a, b));
/// assert_eq!(b_a, cross(b, a));
pub fn cross(a: Vector, b: Vector) -> Vector {
    use Direction::*;

    Vector::new(
        a[Y] * b[Z] - a[Z] * b[Y],
        a[Z] * b[X] - a[X] * b[Z],
        a[X] * b[Y] - a[Y] * b[X],
    )
}

impl Add<Vector> for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub<Vector> for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(std::simd::f64x4::from_array([
            self.0[0] * rhs,
            self.0[1] * rhs,
            self.0[2] * rhs,
            self.0[3],
        ]))
    }
}

impl Div<f64> for Vector {
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

    #[test]
    fn multiply_vector_scalar() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let scalar = 3.0;

        let want = Vector::new(3.0, 6.0, 9.0);

        assert_eq!(want, v * scalar);
    }

    #[test]
    fn magnitude_of_vector() {
        let v = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(1.0, v.mag());

        let v = Vector::new(0.0, 1.0, 0.0);
        assert_eq!(1.0, v.mag());

        let v = Vector::new(0.0, 0.0, 1.0);
        assert_eq!(1.0, v.mag());

        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(f64::sqrt(14.0), v.mag());

        let v = Vector::new(1.0, 2.0, 3.0).neg();
        assert_eq!(f64::sqrt(14.0), v.mag());
    }

    #[test]
    fn normalize_vectors() {
        let v = Vector::new(4.0, 0.0, 0.0);

        let want = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(want, v.norm());
        assert_eq!(1.0, v.norm().mag());

        let v = Vector::new(1.0, 2.0, 3.0);

        let want = Vector::new(
            1.0 / f64::sqrt(14.0),
            2.0 / f64::sqrt(14.0),
            3.0 / f64::sqrt(14.0),
        );
        assert_eq!(want, v.norm());
        assert_eq!(1.0, v.norm().mag())
    }

    #[test]
    fn compute_dot_product() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(20.0, dot(a, b))
    }

    #[test]
    fn compute_cross_product() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);

        let a_b = Vector::new(-1.0, 2.0, -1.0);
        let b_a = Vector::new(1.0, -2.0, 1.0);

        assert_eq!(a_b, cross(a, b));
        assert_eq!(b_a, cross(b, a));
    }
}
