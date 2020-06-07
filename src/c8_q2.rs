// Robot in a Grid: Imagine a robot sitting on the upper left corner of grid with r rows and c columns.
// The robot can only move in two directions, right and down, but certain cells are "off limits" such that
// the robot cannot step on them. Design an algorithm to find a path for the robot from the top left to
// the bottom right.

// _ _ #
// # _ #
// # _ _

enum CellStatus {
    Allowed,
    NotAllowed,
}

impl CellStatus {
    fn can_step(&self) -> bool {
        match *self {
            CellStatus::Allowed => true,
            _ => false,
        }
    }
}

fn find_path(mut maze: Vec<Vec<CellStatus>>) -> Vec<(usize, usize)> {
    let mut path = vec![];
    r_find_path(&mut maze, 0, 0, &mut path);
    path
}

fn r_find_path(
    maze: &mut Vec<Vec<CellStatus>>,
    x: usize,
    y: usize,
    path: &mut Vec<(usize, usize)>,
) -> bool {
    if (x > maze[0].len() - 1 || y > maze.len() - 1) || !maze[x][y].can_step() {
        false
    } else if (x, y) == (maze[0].len() - 1, maze.len() - 1) {
        path.push((x, y));
        true
    } else {
        if r_find_path(maze, x + 1, y, path) || r_find_path(maze, x, y + 1, path) {
            path.push((x, y));
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_path() {
        let maze = vec![
            vec![
                CellStatus::Allowed,
                CellStatus::Allowed,
                CellStatus::NotAllowed,
            ],
            vec![
                CellStatus::NotAllowed,
                CellStatus::Allowed,
                CellStatus::NotAllowed,
            ],
            vec![
                CellStatus::NotAllowed,
                CellStatus::Allowed,
                CellStatus::Allowed,
            ],
        ];

        assert_eq!(
            vec![(2, 2), (2, 1), (1, 1), (0, 1), (0, 0)],
            find_path(maze)
        )
    }
}
