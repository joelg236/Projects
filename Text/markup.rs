use std::env;
use std::fs::File;
use std::io::Read;

fn tag_level(label: &str, level: Option<u8>, contents: &str) -> String {
    match level {
        Some(level) => format!("<{0}{1}>{2}</{0}{1}>", label, level, contents),
        None => format!("<{0}>{1}</{0}>", label, contents)
    }
}

fn tag(label: &str, contents: &str) -> String {
    tag_level(label, None, contents)
}

fn tags_level(label: &str, level: Option<u8>) -> (String, String) {
    match level {
        Some(level) => (format!("<{}{}>", label, level), format!("</{}{}>", label, level)),
        None => (format!("<{}>", label), format!("</{}>", label))
    }
}

fn tags(label: &str) -> (String, String) {
    tags_level(label, None)
}

fn main() {
    let mut file = File::open(env::args().nth(1).expect("Please provide filename")).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("File reading failed");
    let mut lines: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();

    // titles
    lines = lines.iter().map(|line| {
        let mut level: Option<u8> = None;
        for ch in line.chars() {
            match ch {
                '#' => level = Some(level.unwrap_or(0) + 1),
                _ => break
            }
        }
        match level {
            Some(l) => {
                tag_level("h", level, &line[l as usize..].trim_left())
            },
            None => line.to_string()
        }
    }).collect();

    // lists
    let mut in_list = false;
    lines = lines.iter().map(|l| {
        let mut line: String = l.to_string();
        if line.trim_left().starts_with("- ") {
            line = tag("li", &line.trim_left()[2..]);

            if !in_list {
                in_list = true;
                line = tags("ul").0 + "\n" + &line;
            }
        } else if in_list {
            line = line + &tags("ul").1 + "\n";
            in_list = false;
        } else {
            in_list = false;
        }

        line
    }).collect();

    for l in lines {
        println!("{}", l);
    }
}
