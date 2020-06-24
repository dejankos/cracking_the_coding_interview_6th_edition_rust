// Eight Queens:Write an algorithm to print all ways of arranging eight queens on an 8x8 chess board
// so that none of them share the same row, column, or diagonal. In this case, "diagonal" means all
// diagonals, not just the two that bisect the board.

type Position = Vec<Vec<usize>>;

const SIZE: usize = 8;

fn find_positions(row: usize, columns: &mut Vec<usize>, res: &mut Position) {
    if row == SIZE {
        res.push(columns.clone());
    } else {
        for c in 0..SIZE {
            if is_valid(row, c, &columns) {
                columns[row] = c;
                find_positions(row + 1, columns, res)
            }
        }
    }
}

fn is_valid(row: usize, column: usize, columns: &Vec<usize>) -> bool {
    let mut row2 = 0;
    while row2 < row {
        let col2 = columns[row2];
        if col2 == column {
            return false;
        }

        let column_dist = ((col2 as isize) - (column as isize)).abs() as usize;
        let row_dist = row2 - row2;
        if column_dist == row_dist {
            return false;
        }
        row2 += 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_positions() {
        let mut pos = vec![vec![0; SIZE]; SIZE];
        find_positions(0, &mut vec![0; SIZE], &mut pos);
    }
}
