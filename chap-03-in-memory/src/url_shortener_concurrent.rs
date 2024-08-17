// Concurrent version of the url shortener.
// Methods return fully owned copies because any reference to values in the 
// map are only valid for the duration of the lock


use std::collections::HashMap;
use cuid2::CuidConstructor;
use std::sync::RwLock;

pub struct UrlShortenerConcurrent {
    store: RwLock<HashMap<String, String>>,
    id_gen: CuidConstructor
}

impl UrlShortenerConcurrent {

    pub fn new() -> Self {
        UrlShortenerConcurrent {
            store: RwLock::new(HashMap::new()),
            id_gen: CuidConstructor::new()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.store.read().unwrap().is_empty()
    }

    pub fn shorten(&self, path: &str) -> String {
        let mut read_store = self.store.write().unwrap();
        read_store
            .entry(path.to_owned())
            .or_insert(self.id_gen.create_id())
            .clone()
    }

    pub fn link(&self, path: &str) -> Option<String> {
        let read_store = self.store.read().unwrap();
        read_store.get(path).cloned()
    }

}


#[cfg(test)]
mod url_shortener_concurrent_tests {
    use super::UrlShortenerConcurrent;

    #[test]
    fn shortening_a_path_twice_should_yield_the_same_result() {
        let shortener = UrlShortenerConcurrent::new();
        assert!(shortener.is_empty());

        let shortened_1 = shortener.shorten("some/path");
        let shortened_2 = shortener.shorten("some/path");
        assert_eq!(shortened_1, shortened_2);
    }

    #[test]
    fn shortening_2_links_should_yield_different_ids() {
        let shortener = UrlShortenerConcurrent::new();
        let shortened_1 = shortener.shorten("some/path/1");
        let shortened_2 = shortener.shorten("some/path/2");
        assert_ne!(shortened_1, shortened_2);
    }

    #[test]
    fn shortening_a_path_should_then_be_present_in_store() {
        let shortener = UrlShortenerConcurrent::new();
        let shortened = shortener.shorten("some/path");
        let retrieved =  shortener.link("some/path");
        assert_eq!(Some(shortened), retrieved);
    }
}
