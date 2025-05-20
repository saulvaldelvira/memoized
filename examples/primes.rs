use memoized::{memoize, Memoized};

pub fn main() {
    let mut args = std::env::args().skip(1);
    let start = args.next().map(|a| a.parse::<u64>().unwrap()).unwrap_or(0);
    let end = args.next().map(|a| a.parse::<u64>().unwrap());

    let mut is_prime = memoize(|n| {
        for i in 2..n {
            if n % i == 0 {
                return false
            }
        }
        true
    });

    let mut next_prime = memoize(move |mut n: u64| {
        loop {
            n += 1;
            if *is_prime.call(n) {
                return n;
            }
        }
    });

    match end {
        Some(end) => {
            println!("Primes from {start} to {end}");
            let mut n = next_prime.call_cloned(start);
            while n <= end {
                println!("{n}");
                n = next_prime.call_cloned(n);
            }
        }
        None => {
            println!("Next prime from {start} is {}", next_prime.call_cloned(start))
        }
    }

}
