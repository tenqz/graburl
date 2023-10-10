use graburl::parse_url;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let domain = args[1].clone();
    println!("{:?}", parse_url(domain));
}
