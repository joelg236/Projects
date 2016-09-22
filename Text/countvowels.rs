use std::io;

fn main() {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let mut buf = String::new();

    while match io::stdin().read_line(&mut buf) {
        Ok(_) => buf.trim(),
        Err(_) => panic!("read_line failed")
    }.len() != 0 {
        let vs = buf.as_bytes().iter().fold(0,
        |acc, &x| {
            if vowels.contains(&(x as char)) { acc + 1 }
            else { acc }
        });
        println!("{} vowels", vs);
        buf.clear();
    }
}
