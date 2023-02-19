use num_traits::Zero;
use std::ops::{Add, Index, Mul};

#[derive(Clone, Debug)]
pub struct Matrix<T> {
    pub entries: Vec<T>, // row-major order, so entries[..cols-1] would be first row and so on.
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

    /// Divides matrix into 4 square blocks with indices:
    /// [[0], [1]]
    /// [[2], [3]]
    /// Requires the matrix to be square and even-sized, with a minimum size of 4.
    pub fn get_blocks(&self) -> [Matrix<T>; 4] {
        assert!(self.is_square());

        let size = self.rows;
        assert_eq!(size % 2, 0);
        assert!(size >= 4);

        // each chunk represents a block's row, like this:
        // [ [...], [...], ] ^
        // [ [...], [...], ] |
        // [   .      .    ] |
        // [   .      .    ] | n
        // [   .  ,   .  , ] |
        // [ [...], [...], ] |
        // [ [...], [...], ] \
        //   <--->  <--->
        //    n/2    n/2
        // where n = size.
        let select_block = |offset: usize| -> Vec<T> {
            self.entries
                .chunks(size / 2)
                .skip(offset)
                .step_by(2)
                .take(size / 2)
                .flatten()
                .copied()
                .collect()
        };

        // these starting offsets were obtained by tinkering with the previous diagram on paper.
        let blocks = [0, 1, size, size + 1].map(|offset| select_block(offset));

        blocks.map(|entries| Matrix {
            entries,
            rows: size / 2,
            cols: size / 2,
        })
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

    #[test]
    fn test_get_blocks() {
        let matrix = Matrix::new(4, 4, &[1, 2,   3,  4,
                                         5, 6,   7,  8,
                                         9, 10,  11, 12,
                                         13, 14, 15, 16]);

        let blocks = matrix.get_blocks();
        assert_eq!(blocks[0].entries, vec![1, 2,
                                           5, 6]);

        assert_eq!(blocks[1].entries, vec![3, 4,
                                           7, 8]);

        assert_eq!(blocks[2].entries, vec![9,  10,
                                           13, 14]);

        assert_eq!(blocks[3].entries, vec![11, 12,
                                           15, 16]);
    }
}
