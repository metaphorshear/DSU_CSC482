use num::bigint::BigUint;
use num::CheckedAdd;
use num::CheckedMul;
use std::ops::Mul;

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    pub rows : usize,
    pub cols : usize,
    pub array : Vec< Vec<BigUint> >,
}
impl Matrix {
    pub fn new (rows : usize, cols: usize) -> Matrix{
        assert!(rows != 0 && cols != 0); //too confusing to allow this
        Matrix { rows : rows, cols: cols, array: vec![vec![BigUint::from(0u8); cols]; rows] }
    }
    pub fn identity (size : usize) -> Matrix{
        let mut m = Matrix::new(size, size);
        for i in 0..size {
            m.array[i][i] = BigUint::from(1u8);
        }
        m
    }
    pub fn pow(&self, exp: u64) -> Matrix{
        let mut result = Matrix::identity(self.rows);
        let mut mm = self.clone();
        let mut pp = exp;
        while pp > 0 {
           if pp % 2 == 1 { result = &result * &mm; }
           pp = pp / 2;
           mm = &mm * &mm; //works because of impl of Mul trait below
        }
        result
    }
}

impl<'a, 'b> Mul<&'b Matrix> for &'a Matrix {
    type Output = Matrix;
    fn mul(self, rhs: &'b Matrix) -> Matrix {
        assert!(self.cols == rhs.rows);
        let mut result = Matrix::new(self.rows, rhs.cols);
        for i in 0..result.rows {
            for j in 0..result.cols {
                let mut tmp = BigUint::from(0u8);
                for k in 0..self.cols {
                    tmp = tmp.checked_add( &self.array[i][k].checked_mul(&rhs.array[k][j]).unwrap() ).unwrap();
                }
                result.array[i][j] = tmp;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn zero_size_matrix() {
        Matrix::new(0, 0);
    }
    #[test]
    #[should_panic]
    fn bad_matrix_mul() {
        let m = Matrix::new(2, 4);
        let n = Matrix::new(2, 4);
        &m * &n;
    }
    #[test]
    fn identity_pow() {
        let eye = Matrix::identity(3);
        let result = eye.pow(3);
        assert_eq!(result, eye);
    }
}
