use crate::math::num::MatrixF64;

#[derive(Clone, Debug, PartialEq)]
pub struct Affine {
    a: MatrixF64,
    b: Vec<f64>,
}

impl Affine {
    pub fn entity(dim: usize) -> Self {
        return Self { a: MatrixF64::one(dim), b: vec![0f64;dim]};
    }
    pub fn new(a: MatrixF64, b: Vec<f64>) -> Self {
        if a.row_len() != a.col_len() {
            panic!("Size unmatch. a.row:{}, a.col:{}", a.row_len(), a.col_len());
        }
        if a.row_len() != b.len() {
            panic!("Size unmatch. a:{}, b:{}", a.row_len(), b.len());
        }
        return Self { a, b };
    }
    pub fn compose(&self, g: &Self) -> Self {
        if self.a.row_len() != g.a.row_len() {
            panic!("Size unmatch. f:{}, g:{}", self.a.row_len(), g.a.row_len());
        }
        let dim = self.b.len();
        let mut a = MatrixF64::zero(dim,dim);
        for i in 0..dim { for j in 0..dim { for k in 0..dim {
            a[i][j] += g.a[i][k]*self.a[k][j];
        }}}
        let mut b = vec![0f64; dim];
        for i in 0..dim { for j in 0..dim {
            b[i] += g.a[i][j] * self.b[j];
        }}
        for i in 0..dim {
            b[i] += g.b[i];
        }
        return Self { a, b };
    }
    pub fn transform(&self, v: &Vec<f64>) -> Vec<f64> {
        if self.a.row_len() != v.len() {
            panic!("Size unmatch. f:{}, v:{}", self.a.row_len(), v.len());
        }
        let dim = self.b.len();
        let mut nv = vec![0f64; dim];
        for i in 0..dim { for j in 0..dim {
            nv[i] += self.a[i][j] * v[j];
        }}
        for i in 0..dim {
            nv[i] += self.b[i];
        }
        return nv;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_affine() {
        let af = Affine::entity(2);
        let v = vec![3f64,2f64];
        assert_eq!(v, af.transform(&v));
        let af2 = af.compose(&Affine::entity(2));
        assert_eq!(af, af2);
        let m = MatrixF64::from(vec![vec![2f64,0f64],vec![0f64,1f64]]);
        let af3 = Affine::new(m, vec![0f64, 3f64]);
        let v3 = af3.transform(&v);
        assert_eq!(&vec![6f64, 5f64], &v3);
        let m = MatrixF64::from(vec![vec![0f64,2f64],vec![1f64,1f64]]);
        let af4 = Affine::new(m, vec![3f64, 0f64]);
        let v4 = af4.transform(&v);
        assert_eq!(&vec![7f64, 5f64], &v4);
        assert_eq!(vec![13f64, 11f64], af4.transform(&v3));
        let af5 = af3.compose(&af4);
        assert_eq!(vec![13f64, 11f64], af5.transform(&v));
        let af6 = af4.compose(&af3);
        assert_eq!(vec![14f64, 8f64], af6.transform(&v));
    }
}
