use std::io;

fn main() {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let mut buf = String::new();

    while match io::stdin().read_line(&mut buf) {
        Ok(_) => buf.trim(),
        Err(_) => panic!("read_line failed")
    }.len() != 0 {
        let mut piglatin = Vec::new();
        for word in buf.split_whitespace() {
            if vowels.contains(&word.chars().next().unwrap()) {
                piglatin.push(format!("{}-yay", word)) ;
            } else {
                piglatin.push(format!("{}-{}ay", word[1..].to_string(), word.chars().next().unwrap()));
            }
        }
        for word in piglatin.iter() {
            print!("{} ", word);
        }
        print!("\n");
        buf.clear();
    }
}

#[test]
fn test() {
}
