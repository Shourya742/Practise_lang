use std::marker::PhantomData;

pub fn strtok<'a, 'b>(s: &'a mut &'b str, delimiter: char) -> &'b str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i+delimiter.len_utf8()) ..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        fn check_is_static(_: &'static str) {}
        let mut x = "hello world";
        check_is_static(x);
        // &'static mut &'static str
        // &'static mut &'static str
        // let hello = strtok(&mut x, ' ');
        // assert_eq!(hello, "hello");
        // assert_eq!(x, "world");
        let z = &mut x; // This works because reference to mut is covariant, so it can reduce its lifetime
                                   // &'x mut -> &'until-z mut
                                   // until-z: borrow of x stops here
        // &'a/x mut &'static str
        // &'x mut &'static str
        let hello = strtok(&mut x, ' ');
        assert_eq!(hello, "hello");
        assert_eq!(x, "world");
    }
}


// fn main() {
//     let s = String::new(); 
//     let x: &'static str = "hello world";
//     let mut y /* :&'a */ = &*s;
//     y=x; // 'static -> 'a
// }

// T: U
// T is at least as useful as U
// 
// 'static: 'a
// 
// class Animal;
// class Cat: Animal;
// 
// Cat:Animal
// 
// covariance
// 
//     fn foo(&'a str) {}
//     
//     foo(&'a str)
//     foo(&'static str)
// 
// contravariance
//     fn foo(bar: Fn(&'a str) -> ()) {
//          bar("" /* 'a some shorter lifetime a */)
//     }
// 
//     let x: Fn(&'a str) -> ()
// 
//     foo(fn(&'static str) {})
//     
//     &'static str // more useful
//     &'a str
//     
//     'static <: 'a
//     &'static T <: &'a T
// 
//     Fn(&'static str)
//     Fn(&'a str) // more useful
// 
//     'static <: 'a
//     Fn(&'a T) <: Fn(&'static T)
// 
// invariance (Provide exactly whats useful, not more, not less)
// 
// fn foo(s: &mut &'a str, x: &'a str) {
//     *s = x;
// }
// 
// let mut x: &'static str = "hello world";
// let z =  String::new();
// foo(&mut x, &z);
//     // foo(&mut &'a str, &'a str)
//     // foo(&mut &'static str, &'a str)
// drop(z)
// println!("{}",x);
// mutable references are covariant in their lifetime but invariant in their T
// fn bar() {
//     let mut y =  true;
//     let mut z /* &'y mut bool */= &mut y;

//     let x = Box::new(true);
//     let x: &'static mut bool = Box::leak(x);

//     let _ = z;

//     z = x; // &'y mut bool = &'static mut bool

//     drop(z);
// }



struct Deserializer<T> {
    // some fields
    _t: PhantomData<T>
}
/// This is covariant in T
struct Deserializer2<T> {
    // some fields
    _t: PhantomData<fn() -> T>
}
/// This is contravariant in T
struct Deserializer3<T> {
    // some fields
    _t: PhantomData<fn(T)>
}
/// This is invariant in T
struct Deserializer4<T> {
    // some fields
    _t1: PhantomData<fn() -> T>,
    _t2: PhantomData<fn(T)>
}

/// This is invariant in T
struct Deserializer5<T> {
    // some fields
    t1: PhantomData<*mut T>
}

/// This is covariant in T
struct Deserializer6<T> {
    // some fields
    t1: PhantomData<*const T>
}