//! # Embedding HKTs in Rust
//!
//! We're going to start by examining what I believe is currently the only sane encoding of HKTs in
//! Rust. It's used by a number of existing FP libraries. However, it is insufficient for
//! type-level programming and I'll be demonstrating why! Then we'll move on to the "better" encoding,
//! which is much generic programming but has insurmountable performance and usability drawbacks.
//!
//! ```rust
//! trait HKT<T> {
//!    type C;
//!    type T;
//! }
//!
//! impl<T, U> HKT<U> for Option<T> {
//!     type C = T;
//!     type T = Option<U>;
//! }
//!
//! // Functor Definition, we use fmap so that it doesn't conflict with the builtin.
//! trait Functor<U>: HKT<U> {
//!    fn fmap<F>(&self, f: F) -> Self::T where F: Fn(&Self::C) -> U;
//! }
//!
//! // Functor impl for Option
//! impl<T, U> Functor<U> for Option<T> {
//!    fn fmap<F>(&self, f: F) -> Option<U> where F: Fn(&T) -> U {
//!        match *self {
//!            Some(ref value) => Some( f(value) ),
//!            None => None,
//!        }
//!    }
//! }
//! ```
//!
//! Ok, let's try this out:
//! ```rust
//!     # use rust_hkt::part2_alternate::Functor;
//!     assert_eq!(Some(2), Some(1).fmap(|i| i * 2));
//! ```
//!
//! So, this is great, but what about writing functions that take a functor?
//! Let's use our example from earlier.
//!
//! ```
//! # use rust_hkt::part2_alternate::Functor;
//! fn double_in_context<F>(f: F) -> F::T where F: Functor<i32, C=i32> {
//!     f.fmap(|i| i * 2)
//! }
//!
//! assert_eq!(double_in_context(Some(1)), Some(2));
//! ```
//!
//! Cool, I think the interface is really confusing, personally, but it sorta expresses what we want.
//!
//! ```rust
//! # use rust_hkt::part2_alternate::Functor;
//! fn double_and_convert_to_string_in_context<F>(f: F) -> F::T where F: Functor<String, C=i32> {
//!     // let's pretend we have to do this in two steps:
//!     f.fmap(|i| i * 2).fmap(|i| format!("{}", i))
//! }
//! ```

pub trait HKT<T> {
    type C;
    type T;
}

impl<T, U> HKT<U> for Option<T> {
    type C = T;
    type T = Option<U>;
}

// Functor Definition, we use fmap so that it doesn't conflict with the builtin.
pub trait Functor<U>: HKT<U> {
    fn fmap<F>(&self, f: F) -> Self::T
    where
        F: Fn(&Self::C) -> U;
}

// Functor impl for Option
impl<T, U> Functor<U> for Option<T> {
    fn fmap<F>(&self, f: F) -> Option<U>
    where
        F: Fn(&T) -> U,
    {
        match *self {
            Some(ref value) => Some(f(value)),
            None => None,
        }
    }
}
