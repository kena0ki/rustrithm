/// Single-pattern matching with the Knuth-Morris-Pratt algorithm
pub struct Kmp<'a, C: Eq> {
    /// The string pattern to search for.
    pub pattern: &'a [C],
    /// KMP match failure automaton: fail[i] is the length of the longest
    /// string that's both a proper prefix and a proper suffix of pattern[0..=i].
    pub fail: Vec<usize>,
}

impl<'a, C: Eq> Kmp<'a, C> {
    /// Precomputes the automaton that allows linear-time string matching.
    ///
    /// # Example
    ///
    /// ```
    /// use rustrithm::string_proc::Kmp;
    /// let byte_string: &[u8] = b"hello";
    /// let utf8_string: &str = "hello";
    /// let vec_char: Vec<char> = utf8_string.chars().collect();
    ///
    /// let match_from_byte_literal = Kmp::new(byte_string);
    /// let match_from_utf8 = Kmp::new(utf8_string.as_bytes());
    /// let match_from_chars = Kmp::new(&vec_char);
    ///
    /// let vec_int = vec![4, -3, 1];
    /// let match_from_ints = Kmp::new(&vec_int);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if pattern is empty.
    pub fn new(pattern: &'a [C]) -> Self {
        let mut fail = Vec::with_capacity(pattern.len());
        fail.push(0);
        let mut len = 0;
        for ch in &pattern[1..] {
            while len > 0 && pattern[len] != *ch {
                len = fail[len - 1];
            }
            if pattern[len] == *ch {
                len += 1;
            }
            fail.push(len);
        }
        Self { pattern, fail }
    }

    /// KMP algorithm, sets @return[i] = length of longest prefix of pattern
    /// matching a suffix of text[0..=i].
    pub fn kmp_match(&self, text: impl IntoIterator<Item = C>) -> Vec<usize> {
        let mut len = 0;
        text.into_iter()
            .map(|ch| {
                if len == self.pattern.len() {
                    len = self.fail[len - 1];
                }
                while len > 0 && self.pattern[len] != ch {
                    len = self.fail[len - 1];
                }
                if self.pattern[len] == ch {
                    len += 1;
                }
                len
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_kmp_matching() {
        let pattern = "ana";
        let text = "banana";

        let matches = Kmp::new(pattern.as_bytes()).kmp_match(text.bytes());

        assert_eq!(matches, vec![0, 1, 2, 3, 2, 3]);
    }

}
