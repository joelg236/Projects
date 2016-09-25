use std::io;

fn main() {
    let mut buf = String::new();

    while io::stdin().read_line(&mut buf).expect("line read err") > 1 {
        {
            let input = buf.trim();
            println!("{}", input.split_whitespace().count());
        }

        buf.clear();
    }
}

#[test]
fn test() {
}
