#[macro_export]
macro_rules! avec {
    ($($element: expr),* $(,)?) => {{
        /// Check that count is constant
        const _: usize = $crate::avec![@COUNT; $($element),*];
        #[allow(unused_mut)]
        let mut vs = Vec::with_capacity($crate::avec![@COUNT; $($element),*]);
        $(
            vs.push($element);
        )*
        vs
    }};
    ($element: expr;$count: expr) => {{
        let count = $count;
        let mut vs = Vec::with_capacity(count);
        vs.extend(std::iter::repeat($element).take(count));
        vs
    }};

    (@COUNT; $($element:expr),*) => {
        <[()]>::len(&[$($crate::avec![@SUBST; $element]),*])
    };
    (@SUBST; $_element:expr) => { () };
}

trait MaxValue {
    fn max_value()->Self;
}

macro_rules! max_impl {
    ($t: ty) => {
        impl $crate::MaxValue for $t {
            fn max_value() -> Self {
                <$t>::MAX
            }
        }
    };
}

max_impl!(i32);
max_impl!(u32);
max_impl!(i64);


#[test]
fn emtpy_vec() {
    let x: Vec<u32> = avec![];
    assert!(x.is_empty());
}


#[test]
fn single() {
    let x: Vec<u32> = avec![42];
    assert!(!x.is_empty());
    assert_eq!(x.len(),1);
    assert_eq!(x[0],42);
}