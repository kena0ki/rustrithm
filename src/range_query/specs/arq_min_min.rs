use super::ArqSpec;

/// RMQ with range min update.
pub enum ArqMinMin {}
impl ArqSpec for ArqMinMin {
    type S = i64;
    type F = i64;
    fn op(&a: &Self::S, &b: &Self::S) -> Self::S {
        a.min(b)
    }
    fn identity() -> Self::S {
        i64::max_value()
    }
    fn compose(&f: &Self::F, g: &Self::F) -> Self::F {
        f.min(*g)
    }
    fn apply(&f: &Self::F, a: &Self::S, _: i64) -> Self::S {
        f.min(*a)
    }
}

#[cfg(test)]
mod test {
    use crate::range_query::StaticArq;
    use super::ArqMinMin;

    #[test]
    fn test_rmq_rmu() {
        let mut seg = StaticArq::<ArqMinMin>::new(&vec![5;10]);
        seg.update(3,3,&3);
        seg.update(6,6,&3);
        assert_eq!(3,seg.query(3,3));
        assert_eq!(3,seg.query(6,6));
        assert_eq!(5,seg.query(4,5));
        seg.update(3,6,&4);
        assert_eq!(4,seg.query(4,5));
        assert_eq!(3,seg.query(4,6));
        assert_eq!(3,seg.query(3,5));
    }
}

