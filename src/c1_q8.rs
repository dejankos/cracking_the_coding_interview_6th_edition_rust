//Zero Matrix: Write an algorithm such that if an element in an MxN matrix is 0, its entire row and
//column are set to 0.

use std::collections::HashSet;

const M: usize = 4;
const N: usize = 3;

type Matrix = [[usize; M]; N];

fn write_zero(matrix: &mut Matrix) -> &Matrix {
    let mut rows = HashSet::new();
    let mut columns = HashSet::new();

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 0 {
                rows.insert(i);
                columns.insert(j);
            }
        }
    }

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if rows.contains(&i) || columns.contains(&j) {
                matrix[i][j] = 0;
            }
        }
    }


    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_write_zero() {
        let mut matrix: Matrix = [[1, 2, 3, 0], [5, 0, 7, 8], [9, 10, 11, 12]];
        let expected: Matrix = [[0, 0, 0, 0], [0, 0, 0, 0], [9, 0, 11, 0]];
        assert_eq!(write_zero(&mut matrix), &expected);
    }
}