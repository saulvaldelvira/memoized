/*  Copyright (C) 2025 Saúl Valdelvira
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, version 3.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>. */

//! Function memoization
//!
//! # Example
//! ```
//! use memoized::{memoize_rec, Memoized};
//!
//! fn fib(n: u64) -> u64 {
//!     match n {
//!         0 | 1 => n,
//!         _ => fib(n - 1) + fib(n - 2)
//!     }
//! }
//!
//! let mut fib_memo = memoize_rec(|fib, n: u64| {
//!     match n {
//!         0 | 1 => n,
//!         _ => fib(n - 1) + fib(n - 2)
//!     }
//! });
//!
//! for n in 0..20 {
//!     assert_eq!(fib(n), fib_memo.call_cloned(n));
//! }
//! ```

use core::hash::Hash;
use std::collections::HashMap;

/// Represents a memoized function with the signature `(A) -> R`
pub trait Memoized<A, R>
where
    A: Hash + Eq + Copy
{
    /// Calls the function with the given argument.
    ///
    /// If the run in memoized, returns the cached value, else
    /// it computes the result and stores it.
    ///
    /// Returns a reference to the stored result
    fn call(&mut self, arg: A) -> &R;

    /// Shortcut for `self.call(arg).clone()` for
    /// return types that can be cloned
    #[inline]
    fn call_cloned(&mut self, arg: A) -> R
    where
        R: Clone,
    {
        self.call(arg).clone()
    }
}

/// Memoizes the given function
#[inline]
pub fn memoize<F, A, R>(f: F) -> impl Memoized<A, R>
where
    F: FnMut(A) -> R,
    A: Hash + Eq + Copy,
{
    Closure(f, Default::default())
}

/// Memoizes a recursive function.
///
/// This function takes 2 arguments.
/// The first one is a function pointer for recursive calls
/// The second one is the argument of the function
#[inline(always)]
pub fn memoize_rec<A, R, F>(f: F) -> impl Memoized<A, R>
where
    F: Fn(&mut dyn FnMut(A) -> R, A) -> R,
    R: Clone,
    A: Hash + Eq + Copy,
{
    RecursiveClosure(f, Default::default())
}

/// A non-reentrant closure with signature: (A) -> R
pub struct Closure<A, R, F>(F, HashMap<A, R>);

impl<A, R, F> Memoized<A, R> for Closure<A, R, F>
where
    A: Hash + Eq + Copy,
    F: FnMut(A) -> R,
{

    fn call(&mut self, arg: A) -> &R {
        use std::collections::hash_map::Entry;

        let Self(f, mem) = self;

        if let Entry::Vacant(v) = mem.entry(arg) {
            v.insert(f(arg));
        }
        mem.get(&arg).unwrap()
    }
}

/// A recursive closure with signature: ( (A) -> R , A ) -> R
pub struct RecursiveClosure<A, R, F>(F, HashMap<A, R>);

fn call_recursive<'a, A, R, F>(f: &'a F, mem: &'a mut HashMap<A, R>, arg: A) -> &'a R
where
    A: Hash + Eq + Copy,
    R: Clone,
    F: Fn(&mut dyn FnMut(A) -> R, A) -> R,
{
    if mem.contains_key(&arg) {
        mem.get(&arg).unwrap()
    } else {
        let val = f(&mut |a| call_recursive(f, mem, a).clone(), arg);
        mem.entry(arg).or_insert(val);

        mem.get(&arg).unwrap()
    }
}

impl<A, R, F> Memoized<A, R> for RecursiveClosure<A, R, F>
where
    A: Hash + Eq + Copy,
    R: Clone,
    F: Fn(&mut dyn FnMut(A) -> R, A) -> R,
{

    fn call(&mut self, arg: A) -> &R {
        let Self(f, mem) = self;
        call_recursive(f, mem, arg)
    }
}
