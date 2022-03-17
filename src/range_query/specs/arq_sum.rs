use super::ArqSpec;

/// Range Sum Query, a slightly trickier classic application of ARQ.
/// update(l, r, &f) sets all entries a[l..=r] to f.
/// query(l, r) sums all the entries a[l..=r].
///
/// # Panics
///
/// Associated functions will panic on overflow.
//
// Note that while the `size` parameter seems necessary to satisfy the
// Distributive Law, it is merely a convenience: in essence what we've done
// is move to the product monoid of tuples (value, size_of_subtree).
//
// In mathematical jargon, we say that constant assignment f(a) = f is not an
// endomorphism on (i64, +) because f(a+b) = f != 2*f = f(a) + f(b).
// On the other hand, f((a, s)) = (f*s, s) is indeed an endomorphism on pairs
// with vector addition: f((a, s) + (b, t)) = f((a+b, s+t)) = (f*(s+t), s+t)
//                       = (f*s, s) + (f*t, t) = f((a,s)) + f((b,t)).
pub enum ArqSum {}
impl ArqSpec for ArqSum {
    type S = i64;
    type F = i64;
    fn op(&a: &Self::S, &b: &Self::S) -> Self::S {
        a + b
    }
    fn identity() -> Self::S {
        0
    }
    fn compose(&f: &Self::F, _: &Self::F) -> Self::F {
        f
    }
    fn apply(&f: &Self::F, _: &Self::S, size: i64) -> Self::S {
        f * size
    }
}
