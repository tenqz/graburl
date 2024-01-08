pub mod grabber;
pub mod link;

use crate::grabber::Grabber;

pub fn parse_url(start_url: String) -> Vec<String> {
    let grabber = Grabber::new(start_url);
    grabber.show_all_urls()
}
