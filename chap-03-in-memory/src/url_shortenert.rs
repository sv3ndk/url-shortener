// Concurrent version of the url shortener.
// Methods return fully owned copies because any reference to values in the 
// map are only valid for the duration of the lock


use std::collections::HashMap;
use cuid2::CuidConstructor;
use std::sync::RwLock;

pub struct UrlShortener {
    store: RwLock<HashMap<String, String>>,
    id_gen: CuidConstructor
}

impl UrlShortener {

    pub fn new() -> Self {
        UrlShortener {
            store: RwLock::new(HashMap::new()),
            id_gen: CuidConstructor::new()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.store.read().unwrap().is_empty()
    }

    pub fn shorten(&self, path: &str) -> String {
        let mut read_store = self.store.write().unwrap();
        let link_id = self.id_gen.create_id();
        read_store.insert(link_id.clone(), path.to_string());
        link_id.clone()
    }

    pub fn link(&self, id: &str) -> Option<String> {
        let read_store = self.store.read().unwrap();
        read_store.get(id).cloned()
    }

}


#[cfg(test)]
mod url_shortener_tests {
    use super::UrlShortener;

    #[test]
    fn shortening_2_links_should_yield_different_ids() {
        let shortener = UrlShortener::new();
        let link_id_1 = shortener.shorten("some/path/1");
        let link_id_2 = shortener.shorten("some/path/2");
        assert_ne!(link_id_1, link_id_2);
    }

    #[test]
    fn shortening_a_path_should_then_be_present_in_store() {
        let shortener = UrlShortener::new();
        let link_id = shortener.shorten("some/path");
        let retrieved =  shortener.link(&link_id);
        assert_eq!(Some("some/path".to_owned()), retrieved);
    }
}
