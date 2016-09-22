use std::io;

fn main() {
    let mut buf = String::new();

    while io::stdin().read_line(&mut buf).expect("line read err") > 1 {
        {
            let input = buf.trim();
            let mut iter = input.as_bytes().iter();
            let mut reviter = input.as_bytes().iter().rev();
            let p = iter.all(|&x| {
                x == *reviter.next().unwrap()
            });
            println!("{}", p);
        }

        buf.clear();
    }
}

#[test]
fn test() {
}
