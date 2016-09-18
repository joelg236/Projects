use std::env;
use std::iter::FromIterator;

fn main() {
    println!("binary to decimal {}", to_decimal(env::args().nth(1).unwrap()));
    println!("decimal to binary {}", to_binary(env::args().nth(1).unwrap().parse::<u32>().unwrap()));
}

fn to_decimal(binary: String) -> u32 {
    let x: Vec<bool> = binary.as_bytes().iter().map(|c| *c == '1' as u8).rev().collect();

    let mut sum = 0;
    for (i, b) in x.iter().enumerate() {
        if *b {
            sum += 2u32.pow(i as u32);
        }
    }
    sum
}

fn to_binary(mut num: u32) -> String {
    let mut s: Vec<bool> = Vec::new();

    if num == 0 {
        return "0".to_string();
    }

    while num > 0 {
        s.push(num % 2 != 0);
        num /= 2;
    }

    String::from_iter(s.iter().map(|c| if *c { '1' } else { '0' }).rev())
}

#[test]
fn test() {
    assert_eq!(1245, to_decimal("0000010011011101".to_string()));
    assert_eq!(1245, to_decimal("10011011101".to_string()));
    assert_eq!(55, to_decimal("110111".to_string()));
    assert_eq!(2, to_decimal("10".to_string()));
    assert_eq!("10011011101", to_binary(1245));
    assert_eq!("100101111", to_binary(303));
    assert_eq!("110111", to_binary(55));
    assert_eq!("10", to_binary(2));
}
