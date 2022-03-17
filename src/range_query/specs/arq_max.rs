use super::ArqSpec;

pub enum ArqMax {}
impl ArqSpec for ArqMax {
    type S = i64;
    type F = i64;
    fn op(&a: &Self::S, &b: &Self::S) -> Self::S {
        a.max(b)
    }
    fn identity() -> Self::S {
        0
    }
    fn compose(&f: &Self::F, _: &Self::F) -> Self::F {
        f
    }
    fn apply(&f: &Self::F, _: &Self::S, _: i64) -> Self::S {
        f
    }
}

