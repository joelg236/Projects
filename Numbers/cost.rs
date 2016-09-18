use std::env;

fn main() {
    let mut args = env::args();
    let (w, h, c) = (args.nth(1), args.next(), args.next());
    let width = w.expect("no width provided").parse::<u32>().expect("parsing failed");
    let height = h.expect("no height provided").parse::<u32>().expect("parsing failed");
    let cost = c.expect("no cost provided").parse::<u32>().expect("parsing failed");

    println!("{}", (width * height) * cost);
}

#[test]
fn test() {
}
