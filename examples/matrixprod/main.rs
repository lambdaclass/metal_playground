mod matrix;

use crate::matrix::Matrix;
use num_traits::Zero;
use std::ops::{Add, Mul};

/// Parallel computation of a matrix multiplication through a divide-and-conquer algorithm.
/// Only admits square matrices which orders are powers of two.
fn prod_square_twos_pow<T>(ma: Matrix<T>, mb: Matrix<T>) -> Matrix<T>
where
    T: Add + Mul<Output = T> + Zero + Copy,
{
    let blocks_a = divide_matrix_in_blocks(ma);
    let blocks_b = divide_matrix_in_blocks(mb);
    todo!();
}

fn divide_matrix_in_blocks<T>(m: Matrix<T>) -> Vec<Matrix<T>>
where
    T: Add + Mul<Output = T> + Zero + Copy,
{
    assert!(m.is_square());

    let size = m.rows;
    assert!(size.is_power_of_two());

    if size == 2 {
        return vec![m];
    }

    m.get_blocks()
        .to_vec()
        .into_iter()
        .map(|block| divide_matrix_in_blocks(block)) // recursive step
        .flatten() // flattens all the way up
        .collect()
}

fn main() {
    let matrix_a = Matrix::new(4, 4, &[1, 2,   3,  4,
                                       5, 6,   7,  8,
                                       9, 10,  11, 12,
                                       13, 14, 15, 16]);
    let matrix_b = Matrix::new(4, 4, &[2; 16]);
    todo!();
}
