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
    assert_eq!(fibonacci(0), vec![0;0]);
    assert_eq!(fibonacci(1), vec![1]);
    assert_eq!(fibonacci(2), vec![1, 2]);
    assert_eq!(fibonacci(3), vec![1, 2, 3]);
    assert_eq!(fibonacci(4), vec![1, 2, 3, 5]);
    assert_eq!(fibonacci(5), vec![1, 2, 3, 5, 8]);
    assert_eq!(fibonacci(6), vec![1, 2, 3, 5, 8, 13]);
    assert_eq!(fibonacci(7), vec![1, 2, 3, 5, 8, 13, 21]);
}
