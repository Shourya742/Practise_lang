/// A variadic function

fn add(sum: usize, nums: &mut Vec<usize>) -> usize {
    if let Some(n) = nums.pop() {
        return add(sum+n, nums);
    }
    sum
}

// Real variadic functions aren't possible in Rust today, but we can write a declarative macro that behaves this way!

// Level 1: Hard-coded macro_rules
// #[macro_export]
// macro_rules! add {
//     ($n1: expr, $n2: expr) => {
//         $n1+$n2
//     };
//     ($n1: expr, $n2: expr, $n3: expr) => {
//         $n1 + $n2 + $n3
//     };
//     ($n1: expr, $n2: expr, $n3: expr, $n4: expr) => {
//         $n1+$n2+$n3+$n4
//     }
// }

// Level 2: Recognizing a repeating pattern
// This wont work because we'll always end up with an incomplete expression. Our macro would expand to `1+2+`, so
// there would be a trailling `+` at the end, like the compiler tells you if run `cargo expand`
// #[macro_export]
// macro_rules! add {
//     ($($n:expr),*) => {
//         $($n+)*
//     };
// }

// Level 3: Changing the way we think
// It's a little more complex than before, but what I'm doing here is treating the incoming
// text as two separate things: the first is an expression, and the second is everything else
// that follows. This right here is the first piece of a TT muncher: splitting the incoming code 
// into two parts. Our macro doesn't do much, it basically strip out the first argument. 
// #[macro_export]
// macro_rules! add {
//     ($n:expr, $($others:expr),*) => {
//         $n
//     };
// }

//Level 4: Recursive macro calls
// This doesn't compile yet, but it is the second piece of a TT muncher:recursion. There are two problems
// with this currently. The first is that just like regular old recursion, our macro doesn't have a base
// case, so it won't ever terminate.
// #[macro_export]
// macro_rules! add {
//     ($n: expr) => {
//         $n
//     };
//     ($n:expr, $($others:expr)*) => {
//         $n + $crate::add!($($others)*)
//     };
// }

// Level 4: A TT muncher
// Our second problem is that `1, 2` isn't a valid expression in Rust, and so
// when someone calls `add!(1,2,3)`, our recursive rule doesn't actually match
// `2,3` because it isn't an `expr`. The solution to this is to change the
// `expr` into `tt` and remove the comma after it because it is unnecessary now:
#[macro_export]
macro_rules! add {
    ($n: expr) => {
        $n
    };
    ($n:expr, $($others:tt)*) => {
        $n + $crate::add!($($others)*)
    };
}
fn main() {
    // println!("Hello, world!");
    // let mut nums = vec![1, 2, 3, 4, 5];
    // println!("{}", add(0, &mut nums)); 
    let value = add!(1,2,3,4,5,6);
    println!("Value: {value}");
}
