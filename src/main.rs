use ahref::parser::Parser;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let domain = &args[1];
    let resp = reqwest::blocking::get(domain)?;
    let html = resp.text().unwrap();

    let mut parser = Parser::new(html);
    println!("{:?}", parser.parse_links());

    Ok(())
}
