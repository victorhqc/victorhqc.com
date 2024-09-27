use std::collections::{hash_map::Entry, HashMap};
use std::hash::Hash;

pub trait InsertOrPush<K: Eq + Hash, V> {
    fn insert_or_push(&mut self, key: K, value: V) -> ();
}

impl<K: Eq + Hash, V> InsertOrPush<K, V> for HashMap<K, Vec<V>> {
    fn insert_or_push(&mut self, item: K, value: V) -> () {
        let entry = self.entry(item);

        if let Entry::Vacant(e) = entry {
            e.insert(vec![value]);
        } else {
            entry.and_modify(|p| p.push(value));
        }
    }
}
