use memoized::{memoize_rec, Memoized};
use num_bigint::BigUint;

pub fn main() {
    let mut args = std::env::args().skip(1);
    let start = args.next().map(|a| a.parse::<u64>().unwrap());
    let end = args.next().map(|a| a.parse::<u64>().unwrap());

    let range = match start {
        Some(s) => s ..= end.unwrap_or(s),
        None => 0..=20,
    };

    let mut fib = memoize_rec(|fib, n: u64| {
        match n {
            0 | 1 => BigUint::from(n),
            _ => fib(n - 1) + fib(n - 2)
        }
    });

    for n in range {
        println!("fib({n}) = {}", fib.call(n));
    }
}
