use crate::Vector2D;

#[test]
fn dot() {
    let v1 = Vector2D::new(10.0, 5.0);
    let v2 = Vector2D::new(1.5, 2.0);

    let result = Vector2D::dot(v1, v2);

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
    assert_eq!(std::f32::consts::PI / 4.0, r);
}

#[test]
fn angle_f64() {
    let v: Vector2D<f64> = Vector2D::new(2.0, 2.0);
    let r: f64 = v.angle();
    assert_eq!(std::f64::consts::PI / 4.0, r);
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

#[test]
fn f64_as_i32() {
    let fv: Vector2D<f64> = Vector2D::new(10.5, 11.2);
    let iv = fv.as_i32s();
    assert_eq!(Vector2D::new(10, 11), iv);
}

#[test]
fn f32_as_u32() {
    let fv: Vector2D<f32> = Vector2D::new(10.5, 11.2);
    let uv = fv.as_u32s();
    assert_eq!(Vector2D::new(10, 11), uv);
}

#[test]
fn f32_as_u32_bounded() {
    let fv: Vector2D<f32> = Vector2D::new(-10.5, -11.2);
    let uv = fv.as_u32s();
    assert_eq!(Vector2D::new(0, 0), uv);
}

#[test]
fn readme_snippet() {
    // Vectors have fields X and Y, these can be of any type
    let v1: Vector2D<i32> = Vector2D { x: 10, y: 5 };

    // Alternatively you can use new(..) to condense instantiation
    let v2: Vector2D<f64> = Vector2D::new(13.0, 11.5);

    // There are two ways to cast between Vector2Ds, depending on the source and target types.
    //
    // If the target type has a implementation of From<SourceType>, then you can either use
    // source.into_vec2d() or Vector2D::from_vec2d(source).
    assert_eq!(Vector2D::new(10.0, 5.0), v1.into_vec2d());
    assert_eq!(Vector2D::new(10.0, 5.0), Vector2D::from_vec2d(v1));

    // If there is no From or Into implementation, then you're out of luck unless you are using
    // particular primitives, such as i32 and f64. In this case you can use specialised functions,
    // as shown below:
    assert_eq!(Vector2D::new(13, 11), v2.as_i32s());

    // The full list of interoperable primitives is as follows:
    //   - i32, i64, isize
    //   - u32, u64, usize
    //   - f32, f64
    // More can trivially be added in future, if and when there is any demand for such a thing!

    // As primitives generally implement From/Into for lossless casts, an as_Ts() function is not
    // available for those types, and from(..)/into() should be favoured.
    //
    // as_u32s() and similar casts from signed to unsigned primitives will perform bounds checking,
    // so casting (-10.0, 2.0) to a Vector2D<u32> will result in (0, 2).

    // For types with a Add and Mul implementation, the functions dot() and length_squared() are
    // available. For access to length(), normalise(), or angle() however, you must be using either
    // Vector2D<f32> or Vector2D<f64>.
    let v1_len_sq = v1.length_squared();
    let v2_len = v2.length();
    let v2_dir = v2.normalise();

    // Assuming the operator traits are implemented for the types involved, you can add and subtract
    // Vector2Ds from one-another, as well as multiply and divide them with scalar values.
    assert_eq!(v2, v2_dir * v2_len);
    assert_eq!(Vector2D::new(23.0, 16.5),  v2 + v1.into_vec2d()) ;

    // Finally, for any Vector2D<T>, there is an implementation of From<(T, T)> and From<[T; 2]>
    let v4: Vector2D<f64> = Vector2D::new(1.5, 2.3);
    assert_eq!(v4, (1.5, 2.3).into());
    assert_eq!(v4, [1.5, 2.3].into());
}