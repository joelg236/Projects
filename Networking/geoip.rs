extern crate hyper;
extern crate rustc_serialize;

use std::env;
use std::io::Read;
use rustc_serialize::json;

fn main() {
    let client = hyper::Client::new();

    let mut body = String::new();
    let url = format!("https://freegeoip.net/json/{}", env::args().nth(1).expect("please enter ip"));
    client.get(hyper::Url::parse(&url).expect("ip malformed")).send().unwrap()
        .read_to_string(&mut body).unwrap();

    #[derive(RustcDecodable)]
    struct Location {
        country_name: String
    }

    println!("{}", json::decode::<Location>(&body).unwrap().country_name);
}
