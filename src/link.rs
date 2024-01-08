use regex::Regex;

pub struct Link {
    link: String,
}

impl Link {
    pub fn new(link: String) -> Self {
        Link { link }
    }

    pub fn is_local_link(&self) -> bool {
        let regex = Regex::new(r"^(http|https)://").unwrap();

        regex.is_match(&self.link)
    }
}
