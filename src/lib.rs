//! This crate provides Matrix struct for math caculation, like multiplying or adding.
//! This crate doesn't have functions for graphics programing.

/// using operators overloading
use std::ops;

/// Only rectangle Matrices are supported.   
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    value: Vec<Vec<f32>>,
    rows: usize,
    cols: usize
}

impl Matrix {
    /// Create a new Matrix with the given dimensions and initial value.
    pub fn new(rows: usize, cols: usize, init_value: f32) -> Matrix {
        Matrix { value: vec![vec![init_value; cols]; rows], rows, cols }
    }

    /// Create a new Matrix with the given dimensions and initial values from `Vec<f32>`.
    /// Length of the Vector must be divisible by the number of columns without remainder.
    pub fn from_vec(vec: Vec<f32>, rows: usize, cols: usize) -> Matrix {
        assert!(vec.len() % cols == 0); 
        
        let mut value = Vec::new();
        let mut row = Vec::new();
        for i in 0..vec.len() {
            if i % cols == 0 && i != 0 {
                value.push(row.clone());
                row.clear();
            }
            row.push(vec[i]);
        }
        value.push(row);

        Matrix { value, rows: rows, cols: cols }
    }

    /// Create a new Matrix with the given initial values and dimensions from `Vec<Vec<f32>>`.
    /// All columns of Vector must have the same lengh.
    pub fn from_vec_vec(vec: Vec<Vec<f32>>) -> Matrix {
        let cols_len = vec[0].len();
        assert!(vec.iter().all(|x| x.len() == cols_len), "Matrices cannot have columns of different sizes"); 
        Matrix { value: vec.clone(), rows: vec.len(), cols: cols_len }
    }
}

/// Implement the indexing operator [] for the Matrix struct.
impl ops::Index<(usize, usize)> for Matrix {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &f32 {
        &self.value[index.0][index.1]
    }
}

/// Implement the mutable indexing operator [] for the Matrix struct.
impl ops::IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut f32 {
        &mut self.value[index.0][index.1]
    }
}

/// Implement the mul operator * for multipling two matrices.
impl ops::Mul for Matrix {
    type Output = Result<Self, &'static str>;

    fn mul(self, other: Self) -> Result<Self, &'static str> {
        if self.cols != other.rows {
            return Result::Err("This matrix multiplication is undefined (self.rows must eq other.cols)");
        }

        let mut result = Self::new(self.rows, other.cols, 0.0);

        for i in 0..result.rows {
            for j in 0..result.cols {
                let mut index_result = 0.0;
                for k in 0..self.cols {
                    index_result += self[(i, k)] * other[(k, j)];
                }
                result[(i, j)] = index_result;
            }
        }

        Result::Ok(result)
    }
}

/// Implement the mul operator * for multipling matrix and f32.
impl ops::Mul<f32> for Matrix {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        let mut result = Self::new(self.rows, self.cols, 0.0);

        for i in 0..result.rows {
            for j in 0..result.cols {
                result[(i, j)] = self[(i, j)] * other;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_works() {
        let matrix1 = Matrix::from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3);
        let matrix2 = Matrix::from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 3, 2);
        let zero_matrix = matrix1.clone() * 0.0;
        let result = (matrix1 * matrix2).unwrap();

        assert_eq!(result, Matrix::from_vec_vec(vec![vec![22.0, 28.0], vec![49.0, 64.0]]));
        assert_eq!(zero_matrix, Matrix::new(zero_matrix.rows, zero_matrix.cols, 0.0));
    }
}