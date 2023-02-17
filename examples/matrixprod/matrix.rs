use std::ops::{Add, Mul, Index};
use num_traits::Zero;

pub struct Matrix<T: Add + Mul + Zero> {
    entries: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Add + Mul + Zero> Matrix<T> {
    pub fn new(rows: usize, cols: usize, entries: Vec<T>) -> Self {
        assert_eq!(entries.len(), rows * cols, "Tried to create a matrix with the wrong number of entries.");

        Matrix {
            entries,
            rows,
            cols,
        }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, T> {
        self.entries.iter()
    }
}

impl<T: Add + Mul + Zero> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        let idx = self.cols * row + col;

        &self.entries[idx]
    }
}

impl<T: Add + Mul<Output = T> + Zero> Mul for Matrix<T> {
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
