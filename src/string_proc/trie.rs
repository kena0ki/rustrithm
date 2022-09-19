
use std::collections::{hash_map::Entry, HashMap};

/// Prefix trie, easily augmentable by adding more fields and/or methods
pub struct Trie<C: std::hash::Hash + Eq> {
    pub links: Vec<HashMap<C, usize>>,
}

impl<C: std::hash::Hash + Eq> Default for Trie<C> {
    /// Creates an empty trie with a root node.
    fn default() -> Self {
        Self {
            links: vec![HashMap::new()],
        }
    }
}

impl<C: std::hash::Hash + Eq> Trie<C> {
    /// Inserts a word into the trie, and returns the index of its node.
    pub fn insert(&mut self, word: impl IntoIterator<Item = C>) -> usize {
        let mut node = 0;

        for ch in word {
            let len = self.links.len();
            node = match self.links[node].entry(ch) {
                Entry::Occupied(entry) => *entry.get(),
                Entry::Vacant(entry) => {
                    entry.insert(len);
                    self.links.push(HashMap::new());
                    len
                }
            }
        }
        node
    }

    /// Finds a word in the trie, and returns the index of its node.
    pub fn get(&self, word: impl IntoIterator<Item = C>) -> Option<usize> {
        let mut node = 0;
        for ch in word {
            node = *self.links[node].get(&ch)?;
        }
        Some(node)
    }
}

