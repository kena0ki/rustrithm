
use std::{ops::{Add, Index, IndexMut, Mul, Neg, Sub}, fmt::Debug};

pub trait Num:
    Add<Output=Self>+Mul<Output=Self>+Neg<Output=Self>+Sub<Output=Self>
    +Sized+Clone+Copy+Debug+PartialEq
{
    fn zero() -> Self;
    fn one() -> Self;
}
impl Num for f64 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
}
impl Num for i64 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}
pub type MatrixF64=Matrix<f64>;
pub type MatrixI64=Matrix<i64>;

#[derive(Clone, PartialEq, Debug)]
pub struct Matrix<T:Num> {
    cols: usize,
    inner: Box<[T]>,
}
impl <T:Num> Matrix<T> {
    pub fn zero(rows: usize, cols: usize) -> Self {
        let inner = vec![T::zero(); rows * cols].into_boxed_slice();
        Self { cols, inner }
    }
    pub fn one(cols: usize) -> Self {
        let mut matrix = Self::zero(cols, cols);
        for i in 0..cols {
            matrix[i][i] = T::one();
        }
        matrix
    }
    pub fn vector(vec: &[T], as_row: bool) -> Self {
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
    pub fn row_len(&self) -> usize {
        self.inner.len() / self.cols
    }
    pub fn col_len(&self) -> usize {
        self.cols
    }
    pub fn transpose(&self) -> Self {
        let mut matrix = Matrix::zero(self.cols, self.row_len());
        for i in 0..self.row_len() {
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
impl <T:Num> Index<usize> for Matrix<T> {
    type Output = [T];
    fn index(&self, row: usize) -> &Self::Output {
        let start = self.cols * row;
        &self.inner[start..start + self.cols]
    }
}
impl <T:Num> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let start = self.cols * row;
        &mut self.inner[start..start + self.cols]
    }
}
impl <T:Num> Neg for &Matrix<T> {
    type Output = Matrix<T>;
    fn neg(self) -> Self::Output {
        let inner = self.inner.iter().map(|&v| -v).collect();
        Self::Output {
            cols: self.cols,
            inner,
        }
    }
}
impl <T:Num> Add for &Matrix<T> {
    type Output = Matrix<T>;
    fn add(self, other: Self) -> Self::Output {
        let self_iter = self.inner.iter();
        let inner = self_iter
            .zip(other.inner.iter())
            .map(|(&u, &v)| u + v)
            .collect();
        Self::Output {
            cols: self.cols,
            inner,
        }
    }
}
impl <T:Num> Sub for &Matrix<T> {
    type Output = Matrix<T>;
    fn sub(self, other: Self) -> Self::Output {
        let self_iter = self.inner.iter();
        let inner = self_iter
            .zip(other.inner.iter())
            .map(|(&u, &v)| u - v)
            .collect();
        Self::Output {
            cols: self.cols,
            inner,
        }
    }
}
impl <T:Num> Mul<T> for &Matrix<T>  {
    type Output = Matrix<T>;
    fn mul(self, scalar: T) -> Self::Output {
        let inner = self.inner.iter().map(|&v| v * scalar).collect();
        Self::Output {
            cols: self.cols,
            inner,
        }
    }
}
impl <T:Num> Mul for &Matrix<T>  {
    type Output = Matrix<T>;
    fn mul(self, other: Self) -> Self::Output {
        assert_eq!(self.cols, other.row_len());
        let mut matrix = Self::Output::zero(self.row_len(), other.cols);
        for i in 0..self.row_len() {
            for k in 0..self.cols {
                for j in 0..other.cols {
                    matrix[i][j] = matrix[i][j] + (self[i][k] * other[k][j]);
                }
            }
        }
        matrix
    }
}

impl <T:Num> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(v: Vec<Vec<T>>) -> Self {
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
