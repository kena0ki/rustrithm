use super::ArqSpec;

/// Supply & Demand, based on https://codeforces.com/gym/102218/problem/F
/// update(i, i, &(p, o)) increases supply by p and demand by o at time i.
/// query(l, r) computes total supply and demand at times l to r, as well as
//              how much of the supply is subsequently met by the demand.
//
// Note that the apply() operation is only correct when applied to leaf nodes.
// Therefore, update() must only be used in "eager" mode, i.e., with l == r.
// compose() should be unimplemented!() to prevent accidental "lazy" updates.
pub enum ArqSupplyDemand {}
impl ArqSpec for ArqSupplyDemand {
    type S = (i64, i64, i64); // production, orders, sales
    type F = (i64, i64);
    fn op((p1, o1, s1): &Self::S, (p2, o2, s2): &Self::S) -> Self::S {
        let extra = (p1 - s1).min(o2 - s2);
        (p1 + p2, o1 + o2, s1 + s2 + extra)
    }
    fn identity() -> Self::S {
        (0, 0, 0)
    }
    fn compose(_: &Self::F, _: &Self::F) -> Self::F {
        unimplemented!()
    }
    fn apply(&(p_add, o_add): &Self::F, &(p, o, _): &Self::S, s: i64) -> Self::S {
        assert_eq!(s, 1);
        let p = p + p_add;
        let o = o + o_add;
        (p, o, p.min(o))
    }
}
