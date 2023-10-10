use ahref::parser::Parser;

pub struct Grabber {
    start_link: String,
    current_link: String,
}

impl Grabber {
    pub fn new(start_link: String) -> Self {
        Grabber {
            current_link: start_link.clone(),
            start_link,
        }
    }

    pub fn show_all_urls(self) -> Vec<String> {
        let response = self.get_html();
        let mut parser = Parser::new(response);
        parser.parse_links()
    }

    fn get_html(self) -> String {
        match reqwest::blocking::get(self.current_link) {
            Ok(response) => response.text().unwrap(),
            _ => String::from(""),
        }
    }
}
