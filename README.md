# vector2d
A simple and convenient 2D vector type without any external dependencies. If other vector crates are swiss-army knives, vector2d is a spoon; safe, intuitive, and convenient. As an added bonus, you won't run into any excursions with the law using this library thanks to the awfully permissive Unlicense.

## Using vector2d
You probably don't need any documentation to get by with the `Vector2D` type; functions like `dot`, `length`, and `angle` are hopefully all named intuitively enough for you all to get by. If you find yourself stuck, or worry you might be missing out on some convenient features, then look no further than the snippet below!
```rust
use vector2d::Vector2D;

fn main() {
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
    // specific primitives, such as i32 and f64. In this case you can use specialised functions,
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
    // Casts between signed and unsigned primitives will perform bounds checking, so casting 
    // the vector (-10.0, 2.0) to a Vector2D<u32> will result in the vector (0, 2).

    // For types with an Add and Mul implementation, the functions dot() and length_squared() are
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
```