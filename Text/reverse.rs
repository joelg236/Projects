use std::env;

fn main() {
    let string = env::args().nth(1).expect("no string provided");
    println!("{}", string.chars().rev().collect::<String>());
}
