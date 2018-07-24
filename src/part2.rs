//! # Embedding HKTs in Rust
//!
//! We're going to start by examining what I believe is currently the only sane encoding of HKTs in
//! Rust. It's used by a number of existing FP libraries. However, it is insufficient for
//! type-level programming and I'll be demonstrating why! Then we'll move on to the "better" encoding,
//! which is much better for generic programming but has insurmountable performance and usability
//! tradeoffs.
//!
//! The code here is taken from this [gist](https://gist.github.com/14427/af90a21b917d2892eace).
//!
//! ```rust
//! // This is immediately a bit confusing: this interface doesn't tell us very much about what
//! // the types represent, and this matters: if the reader cannot make inferences about
//! // the behavior of interface based on its definition, neither can the compiler, as we will
//! // see.
//! trait HKT<U> {
//!    type C;
//!    type T;
//! }
//!
//! // Looking at a concrete implementation will help a bit:
//! impl<T, U> HKT<U> for Option<T> {
//!     type C = T;
//!     type T = Option<U>;
//! }
//! ```
//!
//! Okay, so we can see that `C` represent our "current" applied type, and that `T` represents
//! our HKT with it's applied type swapped with `U`. But it's still unclear how this is meant
//! to be used. Let's take a look at how we can use this HKT interface in the definition and
//! implementation of `Functor`:
//!
//! ```
//! # use rust_hkt::part2::HKT;
//! trait Functor<U>: HKT<U> {
//!    fn fmap<F>(&self, f: F) -> Self::T where F: Fn(&Self::C) -> U;
//! }
//!
//! impl<T, U> Functor<U> for Option<T> {
//!    fn fmap<F>(&self, f: F) -> Option<U> where F: Fn(&T) -> U {
//!        match *self {
//!            Some(ref value) => Some( f(value) ),
//!            None => None,
//!        }
//!    }
//! }
//! ```
//! TODO: Explain how this works.
//!
//! Ok, let's try this out:
//! ```rust
//!     # use rust_hkt::part2::Functor;
//!     assert_eq!(Some(2), Some(1).fmap(|i| i * 2));
//! ```
//!
//! We can map, but as discussed earlier, these abstractions are pointless unless we can use them
//! nicely in a generic context. Let's see what it's like to write a function that takes a functor:
//!
//! We'll reuse our example from earlier.
//! ```rust
//! # use rust_hkt::part2::Functor;
//! fn double_in_context<F>(f: F) -> F::T where F: Functor<i32, C=i32> {
//! //                                         return type ^^^  ^^^^^ current type
//!     f.fmap(|i| i * 2)
//! }
//!
//! assert_eq!(double_in_context(Some(1)), Some(2));
//! ```
//!
//! Great! Let's ramp up the complexity a bit with an example carefully crafted to break our
//! interface:
//!
//! ```compile_fail
//! # use rust_hkt::part2::Functor;
//! fn double_and_convert_to_string_in_context<F>(f: F) -> F::T where F: Functor<String, C=i32> {
//!     // let's pretend we have to do this in two steps:
//!     f.fmap(|i| i * 2).fmap(|i| format!("{}", i))
//! }
//! ```
//!
//! This fails to compile with two related errors:
//! ```text
//! error[E0308]: mismatched types
//!  --> src/part2.rs:78:16
//!   |
//! 7 |     f.fmap(|i| i * 2).fmap(|i| format!("{}", i))
//!   |                ^^^^^ expected struct `String`, found i32
//!   |
//!   = note: expected type `std::string::String`
//!              found type `i32`
//! ```
//! Problem 1: We've set the return type of our Functor to `String` (or really, `F<String>`),
//! but because we're trying to map twice, we're actually dealing with two different functors:
//! A `Functor<i32, C=i32>`, and a `Functor<String, C=i32>`
//!
//! ```
//! error[E0599]: no method named fmap found for type <F as HKT<String>>::T in the current scope
//!  --> src/part2.rs:78:23
//!   |
//! 7 |     f.fmap(|i| i * 2).fmap(|i| format!("{}", i))
//!   |                       ^^^^
//! ```
//! Problem 2: `fmap` is not defined on HKT<String>::T? Huh? According to our interface, `HKT<String>::T`
//! should be an Option<String>, right? Isn't that a functor?
//!
//! Nope. We're in a generic context, and we don't know that this is an Option. It could be
//! any HKT, and the associated type T of our HKT is not constrained in any way, so the compiler
//! cannot possibly know that the return type of our first fmap is also a functor!
//!
//! We can get around this, in two ways, but neither option is particularly satisfying:
//!
//! 1. We can can add bounds to the function itself. This is painful and ugly, and
//! ```
//! // TODO
//! ```
//!
//! 2. We can modify the definition of functor, and specify that the return type `F::T`
//!
//! ```
//! // TODO
//! ```
//!
//! However, we're just kicking the can further down the road.
//!
//! To put it another way: our functor returns an `F::T`, and we don't know anything at all
//! about `F::T`. However, a functor is an HKT, `F<A>` for which a function, `map`, exists that takes a
//! function, `Fn(A) -> B` and returns an `F<B>`. Importantly, this is saying that our fmap function
//! *must* return the same higher kinded type, `F`.
//!
//! So, we have a failing of abstraction: Our HKT and Functor traits do not fully express the definition
//! of a functor: that the output-kind must be the same as the input-kind.
//!


pub trait HKT<T> {
    type C;
    type T;
}

impl<T, U> HKT<U> for Option<T> {
    type C = T;
    type T = Option<U>;
}

pub trait Functor<U>: HKT<U> {
    fn fmap<F>(&self, f: F) -> Self::T
    where
        F: Fn(&Self::C) -> U;
}

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

pub trait Functor2<U, B>: HKT<U> where Self::T: HKT<B> {

    fn fmap<F>(&self, f: F) -> Self::T
        where
            F: Fn(&Self::C) -> U;
}

impl<T, U, B> Functor2<U, B> for Option<T> where Self::T: Functor<B> {
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

fn functor_2_test() {

}