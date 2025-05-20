#![feature(test)]
extern crate test;

use memoized::{memoize, memoize_rec, Memoized};
use test::Bencher;

const MAX: u64 = 35;

#[bench]
fn fib_recursive(b: &mut Bencher) {
    fn fib(n: u64) -> u64 {
        match n {
            0 | 1 => n,
            _ => fib(n - 1) + fib(n - 2)
        }
    }

    b.iter(|| {
        for i in 0..MAX {
            println!("{}", fib(i));
        }
    });
}

#[bench]
fn fib_recusive_memo(b: &mut Bencher) {
    let mut fib = memoize_rec(|fib, n: u64| {
        match n {
            0 | 1 => n,
            _ => fib(n - 1) + fib(n - 2)
        }
    });

    b.iter(|| {
        for i in 0..MAX {
            println!("{}", fib.call(i));
        }
    });
}

#[bench]
fn fib_iter(b: &mut Bencher) {
    fn fib(n: u64) -> u64 {
        if n == 0 {
            return 0;
        }
        let (mut a, mut b) = (0, 1);
        for _ in 1..n {
            let temp = b;
            b += a;
            a = temp;
        }
        b
    }

    b.iter(|| {
        for i in 0..MAX {
            println!("{}", fib(i));
        }
    });
}

#[bench]
fn fib_iter_memo(b: &mut Bencher) {
    let mut fib = memoize(|n: u64| {
        if n == 0 {
            return 0;
        }
        let (mut a, mut b) = (0, 1);
        for _ in 1..n {
            let temp = b;
            b += a;
            a = temp;
        }
        b
    });

    b.iter(|| {
        for i in 0..MAX {
            println!("{}", fib.call(i));
        }
    });
}

const HUGE_N: u64 = 99999;

#[bench]
fn fib_memo_huge(b: &mut Bencher) {
    let mut fib = memoize_rec(|fib, n: u64| {
        match n {
            0 | 1 => n,
            _ => fib(n - 1) + fib(n - 2)
        }
    });

    b.iter(|| {
        for n in 0..HUGE_N {
            println!("{}", fib.call(n));
        }
    });
}
