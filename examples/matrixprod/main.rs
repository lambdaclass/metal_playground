mod matrix;

use crate::matrix::Matrix;

fn main() {
    let matrix_a = Matrix::new(4, 4, vec![1; 16]);
    let matrix_a = Matrix::new(4, 4, vec![2; 16]);
}
