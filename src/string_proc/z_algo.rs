/// Z algorithm: computes the array Z[..], where Z[i] is the length of the
/// longest text prefix of text[i..] that is **also a prefix** of text.
///
/// It runs in O(n) time, maintaining the invariant that l <= i and
/// text[0..r-l] == text[l..r]. It can be embedded in a larger algorithm,
/// or used for string searching as an alternative to KMP.
///
/// # Example
///
/// ```
/// use rustrithm::string_proc::z_algorithm;
/// let z = z_algorithm(b"ababbababbabababbabababbababbaba");
/// assert_eq!(
///     z,
///     vec![
///         32, 0, 2, 0, 0, 9, 0, 2, 0, 0, 4, 0, 9, 0, 2, 0, 0, 4, 0, 13, 0, 2,
///         0, 0, 8, 0, 2, 0, 0, 3, 0, 1,
///     ],
/// );
/// ```
pub fn z_algorithm(text: &[impl Eq]) -> Vec<usize> {
    let n = text.len();
    let (mut l, mut r) = (1, 1);
    let mut z = Vec::with_capacity(n);
    z.push(n);
    for i in 1..n {
        if r > i + z[i - l] {
            z.push(z[i - l]);
        } else {
            l = i;
            while r < i || (r < n && text[r - i] == text[r]) {
                r += 1;
            }
            z.push(r - i);
        }
    }
    z
}
