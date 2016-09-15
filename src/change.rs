use std::env;

struct Change {
    dollars: u32,
    cents: u32
}

impl Change {
    fn take_dollars(&mut self, dollars: u32) -> u32 {
        let amount = self.dollars / dollars;
        self.dollars -= amount * dollars;
        amount
    }

    fn take_cents(&mut self, cents: u32) -> u32 {
        let amount = self.cents / cents;
        self.cents -= amount * cents;
        amount
    }
}

fn main() {
    let mut args = env::args();
    let cost = args.nth(1).expect("no cost provided");

    let mut change = match cost.find(".") {
        Some(dot) => {
            Change {
                dollars: cost[..dot].parse::<u32>().expect("ill formatted dollars"),
                cents: format!("{:0<2}", &cost[dot+1..])[..2] .parse::<u32>().expect("ill formatted cents")
            }
        },
        None => {
            Change {
                dollars: cost.parse::<u32>().expect("ill formatted dollars"),
                cents: 0
            }
        }
    };

    println!("hundreds {}", change.take_dollars(100));
    println!("fifties {}",  change.take_dollars(50));
    println!("twenties {}", change.take_dollars(20));
    println!("tens {}",     change.take_dollars(10));
    println!("fives {}",    change.take_dollars(5));
    println!("toonies {}",  change.take_dollars(2));
    println!("loonies {}",  change.take_dollars(1));
    println!("quarters {}", change.take_cents(25));
    println!("dimes {}",    change.take_cents(10));
    // i live in canada!
    if change.cents % 5 > 2 {
        change.cents += 3;
    }
    println!("nickles {}",  change.take_cents(5));
}

#[test]
fn test() {
}
