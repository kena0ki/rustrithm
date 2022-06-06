use super::ArqSpec;

pub enum ArqMax {}
impl ArqSpec for ArqMax {
    type S = i64;
    type F = i64;
    fn op(&a: &Self::S, &b: &Self::S) -> Self::S {
        a.max(b)
    }
    fn identity() -> Self::S {
        i64::min_value()
    }
    fn compose(&f: &Self::F, _g: &Self::F) -> Self::F {
        f
        // update max value
        //f.max(*g)
    }
    fn apply(&f: &Self::F, _a: &Self::S, _: i64) -> Self::S {
        f
        // update max value
        //f.max(*a)
    }
}

