use std::io::*;
use std::net::TcpStream;
use std::env;

fn get_data(domain: &str, server: &str) -> Option<(Option<String>, Vec<String>)> {
    let mut stream = TcpStream::connect((&server[..], 43)).unwrap();
    stream.write_all(format!("{}\n", domain).as_bytes()).unwrap();

    let mut whois_server: Option<String> = None;
    let mut data: Vec<String> = Vec::new();

    let reader = BufReader::new(stream);
    for line in reader.lines().filter_map(|l| l.ok()) {
        if line.starts_with("whois:") || line.starts_with("whois server:") {
            whois_server = Some(line.split_whitespace().last().unwrap().to_string());
        }
        data.push(line);
    }

    if data.is_empty() {
        None
    } else {
        Some((whois_server, data))
    }
}

fn get_tld_server(tld: &str) -> Option<String> {
    match get_data(tld, "whois.iana.org") {
        Some(data) => data.0,
        None => None
    }
}

fn get_whois_data(domain: &str) -> Option<String> {
    let tld = domain.split(".").last().unwrap();
    let server = get_tld_server(tld);

    match server {
        Some(server) => {
            match get_data(domain, &server) {
                Some(data) => {
                    let mut all_data = data.1.join("\n");
                    let more_data = match data.0 {
                        Some(whois) => get_data(domain, &whois),
                        None => None
                    };

                    match more_data {
                        Some(data) => all_data.push_str(&data.1.join("\n")),
                        None => {}
                    }

                    return Some(all_data);
                },
                None => None
            }
        },
        None => None
    }
}

fn main() {
    let n = get_whois_data(&env::args().nth(1).expect("please enter domain"));
    println!("{}", n.expect("whois data could not be found"));
}
