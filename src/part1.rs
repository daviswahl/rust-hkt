//! What is a Higher Kinded Type?
//!
//! If Rust had native support for HKTs they would ideally look like
//! this:
//!
//! ```compile_fail
//! trait Functor<F<_>> {
//!     fn map<A, B, Func>(fa: F<A>, func: Func) -> F<B>
//!     where Func: Fn(A) -> B;
//! }
//! ```
//!
//! Here, `F<_>` is a type constructor, you may have seen this example from Haskell: `(* -> *) -> *`.
//! What this is expressing is that `F` is a type that takes another type and produces a concrete type.
//! Eg:
//!   - `(Option -> i32) -> Option<i32>`
//!   - `(Vec -> i32) -> Vec<i32>`
//!
//! A key takeway here is that `F` itself is not a fully concrete type until it's been applied to
//! another type.
//!
//! So, what would a concrete implementation of Functor look like? Let's do Option and Vec:
//!
//! ```compile_fail
//! impl Functor<Option> for Option {
//!     fn map<A, B, Func>(fa: Option<A>, func: Func) -> Option<B>
//!     where Func: Fn(A) -> B {
//!         match fa {
//!             Some(a) -> Some(func(a)),
//!             None    -> None
//!         }
//!     }
//! }
//! ```
//!
//! ```compile_fail
//! impl Functor<Vec> for Vec {
//!     fn map<A, B, Func>(fa: Vec<A>, func: Func) -> Vec<B>
//!     where Func: Fn(A) -> B {
//!         let mut fb: Vec<A> = Vec::with_capacity(fa.len());
//!         for a in fa {
//!             fb.push(a);
//!         }
//!         fb
//!     }
//! }
//! ```
//!
//! If there's one point I want to drive home about HKTs (and type-level programming in general) is that
//! the end goal here is *not* to be able to map over Option: we can already define a map method directly on
//! option and it will work just fine without higher kinded types or having to know what a Functor is.
//!
//! What we *can't* do without higher kinded types is talk *generically* about things that we can map on.
//! We cannot write a function like this:
//!
//! ```compile_fail
//!   fn double_int_in_context<F<_>>(f: F<i32>) -> F<i32>
//!   where F: Functor<F> {
//!     Functor<F>::map(f, |i| i * 2)
//!   }
//!
//!   double_int_in_context(Some(2));   // produces Some(4)
//!   double_int_in_context(vec![1,2]); // produces Vec(2, 4)
//! ```
//!
//! This allows us to parameterize our functions over things that we can map on. We don't need to
//! know *anything* about `F`, except that it's a Functor, and now we can call map on it.
//!
//! Sure, we can require that something implement `IntoIter`, but this really isn't the same thing:
//! TODO EXPLAIN WHY
//!
//! Also, this is just a trivial example: Functor is one of the basic building blocks of type-level
//! functional programming, but there are more complicated tools that allow us to talk generically
//! about complex functionality. Here are some motivating examples, that I've taken from
//! further on in this series, we'll be implementing these later so these are brief.
