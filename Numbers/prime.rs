use std::io;

fn main() {
    let mut primes: Vec<i64> = vec![2];
    let mut buf = String::new();
    let mut num: i64 = 3;

    while match io::stdin().read_line(&mut buf) {
        Ok(_) => buf.trim(),
        Err(_) => panic!("read_line failed")
    }.len() == 0 {
        'next_prime: loop {
            num += 1;

            for prime in &primes {
                if num % prime == 0 {
                    continue 'next_prime;
                }
            }

            primes.push(num);
            break;
        }

        println!("{:?}", primes);
    }
}

#[test]
fn test() {
}
