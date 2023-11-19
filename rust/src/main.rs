//! this is some content for crate-level docs
//!
//! ##Detailed Introduction
//!
//! this is a detailed intro
//!
//! -[x] This crate allows you to create Person objects
//! -[] This crate allows you to create grocery bills
//!
//! #Examples
//!
//! ```
//! let p1 = Person {first_name:"Trevor".to_string()};
//! println!("{}",p1.first_name)
//!
//! ```
//!
//! ~~This is outdated so don't use this.~~
//!
//!
//! This is a really **important** concept!!
//!
//!

pub mod Linked_List;
pub mod args_;
pub mod async_;
pub mod bin_tree;
pub mod bubble_sort;
pub mod clap_args;
pub mod closure;
pub mod d_type_and_var;
pub mod default_trait;
pub mod doubly_linked_list;
pub mod fibonacci;
pub mod flow_and_cond;
pub mod fs_;
pub mod func_and_mod;
pub mod hashmap_;
pub mod hashset_;
pub mod http_;
pub mod iters_;
pub mod match_and_exp;
pub mod merge_sort;
pub mod mpsc_;
pub mod mutex_;
pub mod op_overload;
pub mod option_;
pub mod quick_sort;
pub mod rust_doc;
pub mod scope_thread;
pub mod serde_;
pub mod structure;
pub mod thread_;
pub mod time;
pub mod traits;
pub mod traits_dyn;
pub mod vec_;

///This is the entry point
fn main() {}

// // This module deals with People and Person Objects
// pub mod People {

//     // This struct represent user
//     struct Person {
//         /// This `first_name` field represent the first anem of the person
//         first_name: String,
//         /// This is the `last name` field represent the last name of the person
//         last_name: String,
//     }
// }
