//! String processing algorithms.
use std::cmp::min;
use std::collections::{hash_map::Entry, HashMap, VecDeque};

mod kmp;
mod z_algo;
mod suffix_array;
pub use kmp::*;
pub use z_algo::*;
pub use suffix_array::*;

/// Prefix trie, easily augmentable by adding more fields and/or methods
pub struct Trie<C: std::hash::Hash + Eq> {
    links: Vec<HashMap<C, usize>>,
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

/// Multi-pattern matching with the Aho-Corasick algorithm
pub struct MultiMatcher<C: std::hash::Hash + Eq> {
    /// A prefix trie storing the string patterns to search for.
    pub trie: Trie<C>,
    /// Stores which completed pattern string each node corresponds to.
    pub pat_id: Vec<Option<usize>>,
    /// Aho-Corasick failure automaton. fail[i] is the node corresponding to the
    /// longest prefix-suffix of the node corresponding to i.
    pub fail: Vec<usize>,
    /// Shortcut to the next match along the failure chain, or to the root.
    pub fast: Vec<usize>,
}

impl<C: std::hash::Hash + Eq> MultiMatcher<C> {
    fn next(trie: &Trie<C>, fail: &[usize], mut node: usize, ch: &C) -> usize {
        loop {
            if let Some(&child) = trie.links[node].get(ch) {
                return child;
            } else if node == 0 {
                return 0;
            }
            node = fail[node];
        }
    }

    /// Precomputes the automaton that allows linear-time string matching.
    /// If there are duplicate patterns, all but one copy will be ignored.
    pub fn new(patterns: impl IntoIterator<Item = impl IntoIterator<Item = C>>) -> Self {
        let mut trie = Trie::default();
        let pat_nodes: Vec<usize> = patterns.into_iter().map(|pat| trie.insert(pat)).collect();

        let mut pat_id = vec![None; trie.links.len()];
        for (i, node) in pat_nodes.into_iter().enumerate() {
            pat_id[node] = Some(i);
        }

        let mut fail = vec![0; trie.links.len()];
        let mut fast = vec![0; trie.links.len()];
        let mut q: VecDeque<usize> = trie.links[0].values().cloned().collect();

        while let Some(node) = q.pop_front() {
            for (ch, &child) in &trie.links[node] {
                let nx = Self::next(&trie, &fail, fail[node], &ch);
                fail[child] = nx;
                fast[child] = if pat_id[nx].is_some() { nx } else { fast[nx] };
                q.push_back(child);
            }
        }

        Self {
            trie,
            pat_id,
            fail,
            fast,
        }
    }

    /// Aho-Corasick algorithm, sets @return[i] = node corresponding to
    /// longest prefix of some pattern matching a suffix of text[0..=i].
    pub fn ac_match(&self, text: impl IntoIterator<Item = C>) -> Vec<usize> {
        let mut node = 0;
        text.into_iter()
            .map(|ch| {
                node = Self::next(&self.trie, &self.fail, node, &ch);
                node
            })
            .collect()
    }

    /// For each non-empty match, returns where in the text it ends, and the index
    /// of the corresponding pattern.
    pub fn get_end_pos_and_pat_id(&self, match_nodes: &[usize]) -> Vec<(usize, usize)> {
        let mut res = vec![];
        for (text_pos, &(mut node)) in match_nodes.iter().enumerate() {
            while node != 0 {
                if let Some(id) = self.pat_id[node] {
                    res.push((text_pos + 1, id));
                }
                node = self.fast[node];
            }
        }
        res
    }
}

/// Manacher's algorithm for computing palindrome substrings in linear time.
/// pal[2*i] = odd length of palindrome centred at text[i].
/// pal[2*i+1] = even length of palindrome centred at text[i+0.5].
///
/// # Panics
///
/// Panics if text is empty.
pub fn palindromes(text: &[impl Eq]) -> Vec<usize> {
    let mut pal = Vec::with_capacity(2 * text.len() - 1);
    pal.push(1);
    while pal.len() < pal.capacity() {
        let i = pal.len() - 1;
        let max_len = min(i + 1, pal.capacity() - i);
        while pal[i] < max_len && text[(i - pal[i] - 1) / 2] == text[(i + pal[i] + 1) / 2] {
            pal[i] += 2;
        }
        if let Some(a) = 1usize.checked_sub(pal[i]) {
            pal.push(a);
        } else {
            for d in 1.. {
                let (a, b) = (pal[i - d], pal[i] - d);
                if a < b {
                    pal.push(a);
                } else {
                    pal.push(b);
                    break;
                }
            }
        }
    }
    pal
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_trie() {
        let dict = vec!["banana", "benefit", "banapple", "ban"];

        let trie = dict.into_iter().fold(Trie::default(), |mut trie, word| {
            trie.insert(word.bytes());
            trie
        });

        assert_eq!(trie.get("".bytes()), Some(0));
        assert_eq!(trie.get("b".bytes()), Some(1));
        assert_eq!(trie.get("banana".bytes()), Some(6));
        assert_eq!(trie.get("be".bytes()), Some(7));
        assert_eq!(trie.get("bane".bytes()), None);
    }

    #[test]
    fn test_ac_matching() {
        let dict = vec!["banana", "benefit", "banapple", "ban", "fit"];
        let text = "banana bans, apple benefits.";

        let matcher = MultiMatcher::new(dict.iter().map(|s| s.bytes()));
        let match_nodes = matcher.ac_match(text.bytes());
        let end_pos_and_id = matcher.get_end_pos_and_pat_id(&match_nodes);

        assert_eq!(
            end_pos_and_id,
            vec![(3, 3), (6, 0), (10, 3), (26, 1), (26, 4)]
        );
    }

    #[test]
    fn test_palindrome() {
        let text = "banana";

        let pal_len = palindromes(text.as_bytes());

        assert_eq!(pal_len, vec![1, 0, 1, 0, 3, 0, 5, 0, 3, 0, 1]);
    }
}
