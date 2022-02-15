
use std::{ops::{Add, Index, IndexMut, Mul, Neg, Sub}, fmt::Debug};

#[derive(Clone, PartialEq, Debug)]
pub struct Matrix {
    cols: usize,
    inner: Box<[f64]>,
}
impl Matrix {
    pub fn zero(rows: usize, cols: usize) -> Self {
        let inner = vec![0.0; rows * cols].into_boxed_slice();
        Self { cols, inner }
    }
    pub fn one(cols: usize) -> Self {
        let mut matrix = Self::zero(cols, cols);
        for i in 0..cols {
            matrix[i][i] = 1.0;
        }
        matrix
    }
    pub fn vector(vec: &[f64], as_row: bool) -> Self {
        let cols = if as_row { vec.len() } else { 1 };
        let inner = vec.to_vec().into_boxed_slice();
        Self { cols, inner }
    }
    pub fn pow(&self, mut n: u64) -> Self {
        let mut base = self.clone();
        let mut result = Self::one(self.cols);
        while n > 0 {
            if n % 2 == 1 {
                result = &result * &base;
            }
            base = &base * &base;
            n /= 2;
        }
        result
    }
    pub fn rows(&self) -> usize {
        self.inner.len() / self.cols
    }
    pub fn transpose(&self) -> Self {
        let mut matrix = Matrix::zero(self.cols, self.rows());
        for i in 0..self.rows() {
            for j in 0..self.cols {
                matrix[j][i] = self[i][j];
            }
        }
        matrix
    }
    pub fn recip(&self) -> Self {
        unimplemented!();
    }
}
impl Index<usize> for Matrix {
    type Output = [f64];
    fn index(&self, row: usize) -> &Self::Output {
        let start = self.cols * row;
        &self.inner[start..start + self.cols]
    }
}
impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let start = self.cols * row;
        &mut self.inner[start..start + self.cols]
    }
}
impl Neg for &Matrix {
    type Output = Matrix;
    fn neg(self) -> Matrix {
        let inner = self.inner.iter().map(|&v| -v).collect();
        Matrix {
            cols: self.cols,
            inner,
        }
    }
}
impl Add for &Matrix {
    type Output = Matrix;
    fn add(self, other: Self) -> Matrix {
        let self_iter = self.inner.iter();
        let inner = self_iter
            .zip(other.inner.iter())
            .map(|(&u, &v)| u + v)
            .collect();
        Matrix {
            cols: self.cols,
            inner,
        }
    }
}
impl Sub for &Matrix {
    type Output = Matrix;
    fn sub(self, other: Self) -> Matrix {
        let self_iter = self.inner.iter();
        let inner = self_iter
            .zip(other.inner.iter())
            .map(|(&u, &v)| u - v)
            .collect();
        Matrix {
            cols: self.cols,
            inner,
        }
    }
}
impl Mul<f64> for &Matrix {
    type Output = Matrix;
    fn mul(self, scalar: f64) -> Matrix {
        let inner = self.inner.iter().map(|&v| v * scalar).collect();
        Matrix {
            cols: self.cols,
            inner,
        }
    }
}
impl Mul for &Matrix {
    type Output = Matrix;
    fn mul(self, other: Self) -> Matrix {
        assert_eq!(self.cols, other.rows());
        let mut matrix = Matrix::zero(self.rows(), other.cols);
        for i in 0..self.rows() {
            for k in 0..self.cols {
                for j in 0..other.cols {
                    matrix[i][j] += self[i][k] * other[k][j];
                }
            }
        }
        matrix
    }
}

impl From<Vec<Vec<f64>>> for Matrix {
    fn from(v: Vec<Vec<f64>>) -> Self {
        let row = v.len();
        let col = v[0].len();
        let mut m = Matrix::zero(v.len(), v[0].len());
        for i in 0..row { for j in 0..col {
            m[i][j] = v[i][j];
        }}
        return m;
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_linalg() {
        let zero = Matrix::zero(2, 2);
        let one = Matrix::one(2);
        let rotate_90 = Matrix {
            cols: 2,
            inner: Box::new([0.0, -1.0, 1.0, 0.0]),
        };
        let x_vec = Matrix::vector(&[1.0, 0.0], false);
        let y_vec = Matrix::vector(&[0.0, 1.0], false);
        let x_dot_x = &x_vec.transpose() * &x_vec;
        let x_dot_y = &x_vec.transpose() * &y_vec;

        assert_eq!(x_dot_x, Matrix::one(1));
        assert_eq!(x_dot_x[0][0], 1.0);
        assert_eq!(x_dot_y, Matrix::zero(1, 1));
        assert_eq!(x_dot_y[0][0], 0.0);
        assert_eq!(&one - &one, zero);
        assert_eq!(&one * 0.0, zero);
        assert_eq!(&rotate_90 * &rotate_90, -&one);
        assert_eq!(&rotate_90 * &x_vec, y_vec);
        assert_eq!(&rotate_90 * &y_vec, -&x_vec);
        assert_eq!(&rotate_90 * &(&x_vec + &y_vec), &y_vec - &x_vec);
    }
}
