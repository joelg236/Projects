use std::env;
use std::f64::consts;

fn main() {
    let digit = env::args().nth(1).expect("no digit provided")
        .parse::<u32>().expect("parsing failed");
    println!("{}", nthdigit(consts::PI, digit));
}

fn nthdigit(constant :f64, n :u32) -> i8 {
    (((constant * 10u64.pow(n) as f64) as i64) % 10) as i8
}

#[test]
fn test() {
    assert_eq!(3, nthdigit(consts::PI, 0));
    assert_eq!(1, nthdigit(consts::PI, 1));
    assert_eq!(4, nthdigit(consts::PI, 2));
    assert_eq!(1, nthdigit(consts::PI, 3));
    assert_eq!(5, nthdigit(consts::PI, 4));
    assert_eq!(9, nthdigit(consts::PI, 5));
    assert_eq!(2, nthdigit(consts::PI, 6));
}
