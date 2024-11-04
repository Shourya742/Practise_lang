
pub fn strlen(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}

pub fn strlen2<S>(s:S) -> usize where S: AsRef<str> {
    s.as_ref().len()
}

pub fn foo1() {
    strlen("Hello world"); // &'static str
    strlen(String::from("Hello")); // String: AsRef<str>
}
// if we add Self: Sized on the trait
// we disallow trait object
pub trait Hei {
    // No named type can be added
    // type  Named;
    // no method with no self can be added
    // for dynamic dispatch for trait object,
    // if it has self, then can be added
    // To exclude a method to be part of vtable, add a sized trait to it.
    // fn weird() where Self: Sized;
    fn hei(&self);
}

impl Hei for &str {
    fn hei(&self) {
        println!("hei {}", self);
    }
}

impl Hei for String {
    fn hei(&self) {
        println!("hei {}", self);
    }
}

// pub fn foo() {
//     "J".hei();
// }

// pub fn bar<T: Hei>(h: T) {
//     h.hei();
// }

// pub fn bar2(h: impl Hei) {
//     h.hei();
// }


pub fn strlen1<S: AsRef<str>>(s: S) -> usize {
    s.as_ref().len()
}

pub fn strlen_str(s: String) -> usize {
    s.len()
}

pub fn strlen_dyn(s: Box<dyn AsRef<str>>) -> usize {
    s.as_ref().as_ref().len()
}

pub fn strlen_dyn2(s: &dyn AsRef<str>) -> usize {
    // &dyn AsRef<str>
    // stored in &
    // 1. a pointer to the actual, concrete, implementing type
    // 2. a pointer to a vtable for the referenced trait
    //
    // vtable or virtual dispatch table
    // is a data structure that has pointers to each of the methods for the trait for the type,
    s.as_ref().len()
}

pub fn say_hei(s: &dyn Hei) {
    // &dyn AsRef<str>
    // stored in &
    // 1. a pointer to the actual, concrete, implementing type
    // 2. a pointer to a vtable for the referenced trait
    //
    // vtable or virtual dispatch table
    // is a data structure that has pointers to each of the methods for the trait for the type,
    //
    // dyn Hei, vtable
    //      struct HeiVtable {
    //          hei: *mut Fn(*mut ()),
    //      }
    //
    // &str -> &dyn Hei
    // 1. Pointer to the str
    // 2. &HeiVtable {
    //      hei: &<str as Hei>::hei
    //  }
    // &String -> &dyn Hei
    // 1. pointer to the String
    // 2. &HeiVtable {
    //     hei: &<String as Hei>::hei
    // }
    s.hei();
    // s.vtable.hei(s.pointer)
}

pub fn main() {
    let x: Box<dyn AsRef<str>> = Box::new(String::from("Hello"));
    strlen_dyn(x);
    let y: &dyn AsRef<str> = &"world";
    strlen_dyn2(y);
}

// pub fn bar<H: Hei>(s: &[dyn H]) {
//     for h in s {
//         h.hei();
//     }
// }

// pub fn foo() {
//     bar(&["J", "Jon"]);
//     bar(&[String::from("J"), String::from("JJ")]);
//     bar(&["j", String::from("JJ")]);
// }