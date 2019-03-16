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

    pub fn as_i32s(&self) -> Vector2D<i32> {
        Vector2D {
            x: self.x as i32,
            y: self.y as i32,
        }
    }

    pub fn as_i64s(&self) -> Vector2D<i64> {
        Vector2D {
            x: self.x as i64,
            y: self.y as i64,
        }
    }

    pub fn as_u32s(&self) -> Vector2D<u32> {
        Vector2D {
            x:  f32::max(0.0, self.x) as u32,
            y:  f32::max(0.0, self.y) as u32,
        }
    }

    pub fn as_u64s(&self) -> Vector2D<u64> {
        Vector2D {
            x:  f32::max(0.0, self.x) as u64,
            y:  f32::max(0.0, self.y) as u64,
        }
    }
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

    pub fn as_i32s(&self) -> Vector2D<i32> {
        Vector2D {
            x: self.x as i32,
            y: self.y as i32,
        }
    }

    pub fn as_i64s(&self) -> Vector2D<i64> {
        Vector2D {
            x: self.x as i64,
            y: self.y as i64,
        }
    }

    pub fn as_f32s(&self) -> Vector2D<f32> {
        Vector2D {
            x: self.x as f32,
            y: self.y as f32,
        }
    }

    pub fn as_u32s(&self) -> Vector2D<u32> {
        Vector2D {
            x:  f64::max(0.0, self.x) as u32,
            y:  f64::max(0.0, self.y) as u32,
        }
    }

    pub fn as_u64s(&self) -> Vector2D<u64> {
        Vector2D {
            x:  f64::max(0.0, self.x) as u64,
            y:  f64::max(0.0, self.y) as u64,
        }
    }
}

impl Vector2D<i32> {
    pub fn as_f32s(&self) -> Vector2D<f32> {
        Vector2D {
            x: self.x as f32,
            y: self.y as f32,
        }
    }

    pub fn as_f64s(&self) -> Vector2D<f64> {
        Vector2D {
            x: self.x as f64,
            y: self.y as f64,
        }
    }

    pub fn as_u32s(&self) -> Vector2D<u32> {
        Vector2D {
            x:  i32::max(0, self.x) as u32,
            y:  i32::max(0, self.y) as u32,
        }
    }

    pub fn as_u64s(&self) -> Vector2D<u64> {
        Vector2D {
            x:  i32::max(0, self.x) as u64,
            y:  i32::max(0, self.y) as u64,
        }
    }
}

impl Vector2D<i64> {
    pub fn as_i32s(&self) -> Vector2D<i32> {
        Vector2D {
            x: self.x as i32,
            y: self.y as i32,
        }
    }

    pub fn as_f32s(&self) -> Vector2D<f32> {
        Vector2D {
            x: self.x as f32,
            y: self.y as f32,
        }
    }

    pub fn as_f64s(&self) -> Vector2D<f64> {
        Vector2D {
            x: self.x as f64,
            y: self.y as f64,
        }
    }

    pub fn as_u32s(&self) -> Vector2D<u32> {
        Vector2D {
            x:  i64::max(0, self.x) as u32,
            y:  i64::max(0, self.y) as u32,
        }
    }

    pub fn as_u64s(&self) -> Vector2D<u64> {
        Vector2D {
            x:  i64::max(0, self.x) as u64,
            y:  i64::max(0, self.y) as u64,
        }
    }
}
impl Vector2D<u32> {
    pub fn as_i32s(&self) -> Vector2D<i32> {
        Vector2D {
            x: self.x as i32,
            y: self.y as i32,
        }
    }

    pub fn as_i64s(&self) -> Vector2D<i64> {
        Vector2D {
            x: self.x as i64,
            y: self.y as i64,
        }
    }

    pub fn as_f32s(&self) -> Vector2D<f32> {
        Vector2D {
            x: self.x as f32,
            y: self.y as f32,
        }
    }

    pub fn as_f64s(&self) -> Vector2D<f64> {
        Vector2D {
            x: self.x as f64,
            y: self.y as f64,
        }
    }
}

impl Vector2D<u64> {
    pub fn as_i64s(&self) -> Vector2D<i64> {
        Vector2D {
            x: self.x as i64,
            y: self.y as i64,
        }
    }

    pub fn as_f32s(&self) -> Vector2D<f32> {
        Vector2D {
            x: self.x as f32,
            y: self.y as f32,
        }
    }

    pub fn as_f64s(&self) -> Vector2D<f64> {
        Vector2D {
            x: self.x as f64,
            y: self.y as f64,
        }
    }

    pub fn as_u32s(&self) -> Vector2D<u32> {
        Vector2D {
            x:  self.x as u32,
            y:  self.y as u32,
        }
    }
}

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

#[cfg(test)]
mod test {
    use crate::Vector2D;

    #[test]
    fn dot() {
        let v1 = Vector2D::new(10.0, 5.0);
        let v2 = Vector2D::new(1.5, 2.0);

        let result = Vector2D::dot(&v1, &v2);

        assert_eq!(25.0, result);
    }
    #[test]
    fn from_vec2d() {
        let iv = Vector2D::new(10, 5);
        let fv = Vector2D::from_vec2d(iv);
        assert_eq!(Vector2D::new(10.0, 5.0), fv);
    }

    #[test]
    fn into_vec2d() {
        let iv = Vector2D::new(10, 5);
        let fv = iv.into_vec2d();
        assert_eq!(Vector2D::new(10.0, 5.0), fv);
    }

    #[test]
    fn from_tuple() {
        let ituple = (10, 5);
        let fv = ituple.into();
        assert_eq!(Vector2D::new(10.0, 5.0), fv);
    }

    #[test]
    fn from_array() {
        let arr = [10, 5];
        let fv = arr.into();
        assert_eq!(Vector2D::new(10.0, 5.0), fv);
    }

    #[test]
    fn length_squared() {
        let v = Vector2D::new(10, 5);
        let r = v.length_squared();
        assert_eq!(125, r);
    }

    #[test]
    fn length_f32() {
        let v: Vector2D<f32> = Vector2D::new(3.0, 4.0);
        let r: f32 = v.length();
        assert_eq!(5.0, r);
    }

    #[test]
    fn length_f64() {
        let v: Vector2D<f64> = Vector2D::new(3.0, 4.0);
        let r: f64 = v.length();
        assert_eq!(5.0, r);
    }

    #[test]
    fn angle_f32() {
        let v: Vector2D<f32> = Vector2D::new(2.0, 2.0);
        let r: f32 = v.angle();
        assert_eq!(std::f32::consts::PI/4.0, r);
    }

    #[test]
    fn angle_f64() {
        let v: Vector2D<f64> = Vector2D::new(2.0, 2.0);
        let r: f64 = v.angle();
        assert_eq!(std::f64::consts::PI/4.0, r);
    }

    #[test]
    fn add() {
        let v1 = Vector2D::new(10.0, 5.0);
        let v2 = Vector2D::new(1.5, 2.0);

        let result = v1 + v2;

        assert_eq!(Vector2D::new(11.5, 7.0), result);
    }

    #[test]
    fn add_assign() {
        let mut v1 = Vector2D::new(10.0, 5.0);
        let v2 = Vector2D::new(1.5, 2.0);

        v1 += v2;

        assert_eq!(Vector2D::new(11.5, 7.0), v1);
    }
    #[test]
    fn sub() {
        let v1 = Vector2D::new(10.0, 5.0);
        let v2 = Vector2D::new(1.5, 2.0);

        let result = v1 - v2;

        assert_eq!(Vector2D::new(8.5, 3.0), result);
    }

    #[test]
    fn sub_assign() {
        let mut v1 = Vector2D::new(10.0, 5.0);
        let v2 = Vector2D::new(1.5, 2.0);

        v1 -= v2;

        assert_eq!(Vector2D::new(8.5, 3.0), v1);
    }

    #[test]
    fn mul() {
        let v = Vector2D::new(10.0, 5.0);
        let f = 2.0;

        let result = v * f;

        assert_eq!(Vector2D::new(20.0, 10.0), result);
    }

    #[test]
    fn mul_assign() {
        let mut v = Vector2D::new(10.0, 5.0);
        let f = 2.0;

        v *= f;

        assert_eq!(Vector2D::new(20.0, 10.0), v);
    }

    #[test]
    fn div() {
        let v = Vector2D::new(10.0, 5.0);
        let f = 2.0;

        let result = v / f;

        assert_eq!(Vector2D::new(5.0, 2.5), result);
    }

    #[test]
    fn div_assign() {
        let mut v = Vector2D::new(10.0, 5.0);
        let f = 2.0;

        v /= f;

        assert_eq!(Vector2D::new(5.0, 2.5), v);
    }
}
