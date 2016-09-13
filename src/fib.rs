use std::env;

fn main() {
    let digit = env::args().nth(1).expect("no digit provided")
        .parse::<usize>().expect("parsing failed");
    println!("{:?}", fibonacci(digit));
}

fn fibonacci(n: usize) -> Vec<i64> {
    let mut seq: Vec<i64> = vec![1, 2];
    seq.truncate(n);
    for i in 2..n {
        let (ll, l) = (seq[i - 2], seq[i - 1]);
        seq.push(ll + l);
    }
    seq
}

#[test]
fn test() {
}
