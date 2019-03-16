#[cfg(test)]
mod test;

use proc_vector2d::{fn_lower_bounded_as, fn_simple_as};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Vector2D<T> {
    pub x: T,
    pub y: T,
}

impl<T: Copy + Clone> Vector2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn from_vec2d<U: Into<T> + Copy + Clone>(src: Vector2D<U>) -> Vector2D<T> {
        Vector2D {
            x: src.x.into(),
            y: src.y.into(),
        }
    }

    pub fn into_vec2d<U: From<T>>(&self) -> Vector2D<U> {
        Vector2D {
            x: self.x.into(),
            y: self.y.into(),
        }
    }
}

impl<T, U, V> Vector2D<T>
where
    T: Mul<T, Output = U> + Copy + Clone,
    U: Add<U, Output = V> + Copy + Clone,
{
    pub fn dot(v1: &Self, v2: &Self) -> V {
        v1.x * v2.x + v1.y * v2.y
    }

    pub fn length_squared(&self) -> V {
        self.x * self.x + self.y * self.y
    }
}

// From Implementations

impl<T, U> From<(U, U)> for Vector2D<T>
where
    T: From<U>,
    U: Copy + Clone,
{
    fn from(src: (U, U)) -> Vector2D<T> {
        Vector2D {
            x: src.0.into(),
            y: src.1.into(),
        }
    }
}

impl<T, U> From<[U; 2]> for Vector2D<T>
where
    T: From<U>,
    U: Copy + Clone,
{
    fn from(src: [U; 2]) -> Vector2D<T> {
        Vector2D {
            x: src[0].into(),
            y: src[1].into(),
        }
    }
}

// Specific Primitive Implementations

impl Vector2D<f32> {
    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    pub fn normalise(&self) -> Self {
        self / self.length()
    }

    pub fn angle(&self) -> f32 {
        self.y.atan2(self.x)
    }

    fn_simple_as!(i32);
    fn_simple_as!(i64);
    fn_simple_as!(isize);
    fn_lower_bounded_as!(f32, u32, 0.0);
    fn_lower_bounded_as!(f32, u64, 0.0);
    fn_lower_bounded_as!(f32, usize, 0.0);
}

impl Vector2D<f64> {
    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn normalise(&self) -> Self {
        self / self.length()
    }

    pub fn angle(&self) -> f64 {
        self.y.atan2(self.x)
    }

    fn_simple_as!(i32);
    fn_simple_as!(i64);
    fn_simple_as!(isize);
    fn_simple_as!(f32);
    fn_lower_bounded_as!(f64, u32, 0.0);
    fn_lower_bounded_as!(f64, u64, 0.0);
    fn_lower_bounded_as!(f64, usize, 0.0);
}

impl Vector2D<i32> {
    fn_simple_as!(isize);
    fn_simple_as!(f32);
    fn_simple_as!(f64);
    fn_lower_bounded_as!(i32, u32, 0);
    fn_lower_bounded_as!(i32, u64, 0);
    fn_lower_bounded_as!(i32, usize, 0);
}

impl Vector2D<i64> {
    fn_simple_as!(i32);
    fn_simple_as!(isize);
    fn_simple_as!(f32);
    fn_simple_as!(f64);
    fn_lower_bounded_as!(i64, u32, 0);
    fn_lower_bounded_as!(i64, u64, 0);
    fn_lower_bounded_as!(i64, usize, 0);
}

impl Vector2D<isize> {
    fn_simple_as!(i32);
    fn_simple_as!(i64);
    fn_simple_as!(f32);
    fn_simple_as!(f64);
    fn_lower_bounded_as!(isize, u32, 0);
    fn_lower_bounded_as!(isize, u64, 0);
    fn_lower_bounded_as!(isize, usize, 0);
}

impl Vector2D<u32> {
    fn_simple_as!(i32);
    fn_simple_as!(i64);
    fn_simple_as!(isize);
    fn_simple_as!(f32);
    fn_simple_as!(f64);
    fn_simple_as!(usize);
}

impl Vector2D<u64> {
    fn_simple_as!(i32);
    fn_simple_as!(i64);
    fn_simple_as!(isize);
    fn_simple_as!(f32);
    fn_simple_as!(f64);
    fn_simple_as!(u32);
    fn_simple_as!(usize);
}

impl Vector2D<usize> {
    fn_simple_as!(i32);
    fn_simple_as!(i64);
    fn_simple_as!(isize);
    fn_simple_as!(f32);
    fn_simple_as!(f64);
    fn_simple_as!(u32);
    fn_simple_as!(u64);
}

// Ops Implementations

impl<T, O> Add<Vector2D<T>> for Vector2D<T>
where
    T: Add<T, Output = O> + Copy + Clone,
{
    type Output = Vector2D<O>;
    fn add(self, rhs: Vector2D<T>) -> Self::Output {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T, O> Add<&Vector2D<T>> for &Vector2D<T>
where
    T: Add<T, Output = O> + Copy + Clone,
{
    type Output = Vector2D<O>;
    fn add(self, rhs: &Vector2D<T>) -> Self::Output {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> AddAssign<Vector2D<T>> for Vector2D<T>
where
    T: Add<T, Output = T> + Copy + Clone,
{
    fn add_assign(&mut self, rhs: Vector2D<T>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<T, O> Sub<Vector2D<T>> for Vector2D<T>
where
    T: Sub<T, Output = O> + Copy + Clone,
{
    type Output = Vector2D<O>;
    fn sub(self, rhs: Vector2D<T>) -> Self::Output {
        Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T, O> Sub<&Vector2D<T>> for &Vector2D<T>
where
    T: Sub<T, Output = O> + Copy + Clone,
{
    type Output = Vector2D<O>;
    fn sub(self, rhs: &Vector2D<T>) -> Self::Output {
        Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> SubAssign<Vector2D<T>> for Vector2D<T>
where
    T: Sub<T, Output = T> + Copy + Clone,
{
    fn sub_assign(&mut self, rhs: Vector2D<T>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl<T, O> Mul<T> for Vector2D<T>
where
    T: Mul<T, Output = O> + Copy + Clone,
{
    type Output = Vector2D<O>;
    fn mul(self, rhs: T) -> Self::Output {
        Vector2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T, O> Mul<T> for &Vector2D<T>
where
    T: Mul<T, Output = O> + Copy + Clone,
{
    type Output = Vector2D<O>;
    fn mul(self, rhs: T) -> Self::Output {
        Vector2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> MulAssign<T> for Vector2D<T>
where
    T: Mul<T, Output = T> + Copy + Clone,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl<T, O> Div<T> for Vector2D<T>
where
    T: Div<T, Output = O> + Copy + Clone,
{
    type Output = Vector2D<O>;
    fn div(self, rhs: T) -> Self::Output {
        Vector2D {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T, O> Div<T> for &Vector2D<T>
where
    T: Div<T, Output = O> + Copy + Clone,
{
    type Output = Vector2D<O>;
    fn div(self, rhs: T) -> Self::Output {
        Vector2D {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T> DivAssign<T> for Vector2D<T>
where
    T: Div<T, Output = T> + Copy + Clone,
{
    fn div_assign(&mut self, rhs: T) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}
