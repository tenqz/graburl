use ahref::parser::Parser;
use regex::Regex;
use crate::link::Link;

pub struct Grabber {
    base_link: String,
    current_link: String,
}

impl Grabber {
    pub fn new(start_link: String) -> Self {
        Grabber {
            current_link: start_link.clone(),
            base_link: Grabber::get_base_link(&start_link),
        }
    }

    pub fn show_all_urls(&self) -> Vec<String> {
        let response = self.get_html();
        let mut parser = Parser::new(response);
        let links = parser.parse_links();
        let mut full_links = Vec::new();

        for link in links {
            if Link::new(link.clone()).is_local_link() {
                full_links.push(format!("{}", link));
            } else {
                full_links.push(format!("{}{}", &self.base_link, link));
            }
        }

        full_links
    }

    fn get_html(&self) -> String {
        match reqwest::blocking::get(&self.current_link) {
            Ok(response) => response.text().unwrap(),
            _ => String::from(""),
        }
    }

    pub fn get_base_link(link: &String) -> String {
        let regex_string = r"^https?:\/\/[^#?\/]+";
        let regex_link = Regex::new(regex_string).unwrap();

        regex_link.find(link).unwrap().as_str().to_string()
    }
}
