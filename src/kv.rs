use std::collections::HashMap;

#[derive(Default)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        unimplemented!()
    }

    pub fn set(&self, key: String, value: String) {
        unimplemented!()
    }

    pub fn get(&self, key: String) -> Option<String> {
        unimplemented!()
    }

    pub fn remove(&self, key: String) {
        unimplemented!()
    }
}
