use std::env;

fn main() {
    let digit = env::args().nth(1).expect("no digit provided")
        .parse::<i64>().expect("parsing failed");
    println!("{:?}", primes(digit));
}

fn factors(n: i64) -> Vec<i64> {
    let mut factors: Vec<i64> = Vec::new();
    for i in 1..n+1 {
        if n % i == 0 {
            factors.push(i);
        }
    }

    factors
}

fn primes(n: i64) -> Vec<i64> {
    let mut primes: Vec<i64> = Vec::new();
    for i in factors(n) {
        if factors(i).len() == 2 {
            primes.push(i);
        }
    }

    primes
}

#[test]
fn test() {
}
