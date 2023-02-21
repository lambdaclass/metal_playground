use num_traits::Zero;
use std::ops::{Add, Index, Mul};

#[derive(Clone, Debug)]
pub struct Matrix<T> {
    pub entries: Vec<T>, // row-major order
    pub rows: usize,
    pub cols: usize,
}

impl<T: Copy> Matrix<T> {
    pub fn new(rows: usize, cols: usize, entries: &[T]) -> Self {
        assert_eq!(
            entries.len(),
            rows * cols,
            "Tried to create a matrix with the wrong number of entries."
        );

        Matrix {
            entries: entries.to_vec(),
            rows,
            cols,
        }
    }

    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }

    pub fn sizeof_entries(&self) -> u64 {
        (self.entries.len() * core::mem::size_of::<T>()) as u64
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        let idx = self.cols * row + col;

        &self.entries[idx]
    }
}

impl<T> Mul for Matrix<T>
where
    T: Add + Mul<Output = T> + Zero + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.cols, rhs.rows);
        let sum_count = self.cols;

        let rows = self.rows;
        let cols = rhs.cols;
        let size = rows * cols;

        let mut entries = Vec::with_capacity(size);

        for idx in 0..size {
            let i = idx / rows;
            let j = idx % cols;

            let mut sum = T::zero();
            for di in 0..sum_count {
                sum = sum + self[(i, di)] * rhs[(di, j)];
            }
            entries.push(sum);
        }

        Matrix {
            rows,
            cols,
            entries,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiplication() {
        let matrix = Matrix::new(2, 2, &[3, 1, 5, 2]);
        let inverse_matrix = Matrix::new(2, 2, &[2, -1, -5, 3]);

        let result = matrix * inverse_matrix;

        assert_eq!(result.entries, vec![1, 0, 0, 1]);
    }
}
