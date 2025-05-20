#![feature(test)]
extern crate test;

use memoized::{memoize, Memoized};
use test::Bencher;

const MAX: u64 = 1000;

fn is_prime(n: u64) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false
        }
    }
    true
}

#[bench]
fn next_prime_normal(b: &mut Bencher) {
    fn next_prime(mut n: u64) -> u64 {
        loop {
            n += 1;
            if is_prime(n) {
                return n;
            }
        }
    }

    b.iter(|| {
        for i in 0..MAX {
            println!("{}", next_prime(i));
        }
    });
}

#[bench]
fn next_prime_memo(b: &mut Bencher) {
    let mut is_prime = memoize(is_prime);

    let mut next_prime = memoize(move |mut n: u64| {
        loop {
            n += 1;
            if *is_prime.call(n) {
                return n;
            }
        }
    });

    b.iter(|| {
        for i in 0..MAX {
            println!("{}", next_prime.call(i));
        }
    });
}

