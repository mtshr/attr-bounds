use attr_bounds::bounds;

#[test]
fn empty_bounds() {
    #[bounds()]
    struct Wrapper<T>(T);

    let _ = Wrapper(42);
}

#[test]
fn enum_bounds() {
    #[bounds(A: std::fmt::Display)]
    #[bounds(B: std::ops::Add<Output = B>)]
    #[derive(Debug)]
    enum Either<A, B> {
        Left(A),
        Right(B),
    }

    let left = Either::<&str, i32>::Left("foo");
    let right = Either::<&str, i32>::Right(42);
    assert_eq!(format!("{left:?}"), "Left(\"foo\")");
    assert_eq!(format!("{right:?}"), "Right(42)");
}

#[test]
fn fn_bounds() {
    #[bounds(T: Clone)]
    fn clone<T>(var: &T) -> T {
        var.clone()
    }

    let v1 = vec![1, 2, 3];
    let v2 = clone(&v1);
    assert_eq!(v1, v2);
}

#[test]
fn impl_bounds() {
    use std::{fmt::Display, ops::Add};

    trait Foo<T>
    where
        T: Copy + Display + Add<Output = T>,
    {
        fn foo(self, x: T) -> String;
    }

    #[bounds(S: Display)]
    #[bounds(T: Copy + Display + Add<Output = T>)]
    impl<S, T> Foo<T> for S {
        fn foo(self, x: T) -> String {
            self.to_string() + &(x + x).to_string()
        }
    }

    assert_eq!(42i32.foo(42i64), "4284");
}

#[test]
fn struct_bounds() {
    #[bounds(T: std::ops::Add<Output = T>)]
    #[derive(Debug)]
    struct Wrapper<T>(T);

    let var = Wrapper(42);
    assert_eq!(format!("{var:?}"), "Wrapper(42)");
}

#[test]
fn trait_bounds() {
    #[bounds(Self: Clone + std::ops::Mul<Output = Self>)]
    trait Square {
        fn square(&self) -> Self;
    }

    impl Square for i32 {
        fn square(&self) -> Self {
            self * self
        }
    }

    assert_eq!(42i32.square(), 42 * 42);
}

#[test]
fn union_bounds() {
    #[bounds(A: Copy)]
    #[bounds(B: Copy)]
    union Union<A, B> {
        a: A,
        b: B,
    }

    let _ = Union::<i32, u8> { a: 42 };
    let _ = Union::<i32, u8> { b: 255 };
}
