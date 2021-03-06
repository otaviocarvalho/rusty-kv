use std::collections::HashMap;

/// KvStore persists key/value entries.
///
/// Entries are kept in memory and not persisted into disk.
#[derive(Default)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// Creates a new KvStore
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    /// Sets a new entry into the KvStore
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// Gets an entry from the KvStore
    pub fn get(&self, key: String) -> Option<String> {
        return self.map.get(&key).cloned();
    }

    /// Removes an entry from the KvStore
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
