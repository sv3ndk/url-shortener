// Basic version of the URL shortener
// Not used: since we're using Tokio, we need the concurrent version

use std::collections::HashMap;

use cuid2::CuidConstructor;


pub struct UrlShortener {
    // fully owning the content since acting as a DB
    store: HashMap<String, String>,
    id_gen: CuidConstructor
}

impl UrlShortener {
    pub fn new() -> Self {
        UrlShortener {
            store: HashMap::new(),
            id_gen: CuidConstructor::new().with_length(10)
        }
    }

    pub fn shorten(&mut self, path: &str) -> String {
        self.store
            .entry(path.to_owned())
            .or_insert(self.id_gen.create_id())
            .to_owned()
    }

    pub fn link(&self, path: &str) -> Option<&str> {
        self.store.get(path).map(|s| s.as_str())
    }
}


#[cfg(test)]
mod url_shortener_tests {
    use super::UrlShortener;

    #[test]
    fn shortening_a_path_twice_should_yield_the_same_result() {
        let mut shortener = UrlShortener::new();
        assert!(shortener.store.is_empty());

        let shortened_1 = shortener.shorten("some/path");
        let shortened_2 = shortener.shorten("some/path");
        assert_eq!(shortened_1, shortened_2);
    }

    #[test]
    fn shortening_2_links_should_yield_different_ids() {
        let mut shortener = UrlShortener::new();
        let shortened_1 = shortener.shorten("some/path/1");
        let shortened_2 = shortener.shorten("some/path/2");
        assert_ne!(shortened_1, shortened_2);
    }

    #[test]
    fn shortening_a_path_should_then_be_present_in_store() {
        let mut shortener = UrlShortener::new();

        let shortened = shortener.shorten("some/path");
        let retrieved =  shortener.link("some/path");

        assert_eq!(Some(shortened.as_str()), retrieved);
    }
}
