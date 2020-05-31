// Jigsaw: Implement an NxN jigsaw puzzle. Design the data structures and explain an algorithm to
// solve the puzzle. You can assume that you have a fitsWith method which, when passed two
// puzzle edges, returns true if the two edges belong together.

const M: usize = 3;
const N: usize = 3;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Orientation {
    Left,
    Top,
    Right,
    Down,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Edge {
    Flat,
    Outer,
    Inner,
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Piece {
    left: Edge,
    top: Edge,
    right: Edge,
    down: Edge,
    orientation: Orientation,
}

#[derive(Debug)]
struct Puzzle {
    matrix: Vec<Vec<Option<Piece>>>,
    pieces: Vec<Piece>,
}

impl Orientation {
    fn rotate_right(&self) -> Orientation {
        match *self {
            Orientation::Left => Orientation::Top,
            Orientation::Top => Orientation::Right,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
        }
    }
}

impl Edge {
    fn is_opposite(&self, other: Edge) -> bool {
        match *self {
            Edge::Outer => other == Edge::Inner,
            Edge::Inner => other == Edge::Outer,
            _ => false,
        }
    }
}

impl Piece {
    fn new(left: Edge, top: Edge, right: Edge, down: Edge, orientation: Orientation) -> Self {
        Piece {
            left,
            top,
            right,
            down,
            orientation,
        }
    }

    fn rotate_right(&mut self) {
        let (l, t, r, d) = (self.down, self.left, self.top, self.right);

        self.left = l;
        self.top = t;
        self.right = r;
        self.down = d;
        self.orientation = self.orientation.rotate_right();
    }

    fn is_corner(&self) -> bool {
        self.flat_parts() == 2
    }

    fn is_edge(&self) -> bool {
        self.flat_parts() == 1
    }

    fn is_inner(&self) -> bool {
        self.flat_parts() == 0
    }

    fn flat_parts(&self) -> usize {
        let mut c = (self.down == Edge::Flat) as usize;
        c = c + (self.top == Edge::Flat) as usize;
        c = c + (self.right == Edge::Flat) as usize;
        c = c + (self.down == Edge::Flat) as usize;

        c
    }
}

impl Puzzle {
    fn new(h: usize, w: usize, pieces: Vec<Piece>) -> Self {
        let matrix = vec![vec![None; w]; h];
        Puzzle { matrix, pieces }
    }

    fn solve(&mut self) {
        let mut corders = self.pieces.clone();
        corders.retain(|p| p.is_edge());
        let mut edges = self.pieces.clone();
        edges.retain(|p| p.is_corner());
        let mut inner = self.pieces.clone();
        inner.retain(|p| p.is_inner());

        for i in 0..self.matrix.len() {
            for j in 0..self.matrix[i].len() {
                let p = self.find_piece(&self.pieces, i, j);
                let idx = self.pieces.iter().position(|x| x == &p.1).unwrap();
                self.pieces.remove(idx);
                self.matrix[i][j] = Some(p.0)
            }
        }
    }

    fn find_piece(&self, pieces: &Vec<Piece>, i: usize, j: usize) -> (Piece, Piece) {
        match (i, j) {
            (0, 0) => find_matching_piece(pieces, |p| p.left == Edge::Flat && p.top == Edge::Flat),
            (0, M) => find_matching_piece(pieces, |p| {
                let left = self.matrix[i][j - 1].unwrap();
                left.right.is_opposite(p.left) && p.right == Edge::Flat && p.top == Edge::Flat
            }),
            (N, 0) => find_matching_piece(pieces, |p| p.left == Edge::Flat && p.down == Edge::Flat),
            (_, 0) => find_matching_piece(pieces, |p| {
                let top = self.matrix[i - i][j].unwrap();
                p.left == Edge::Flat && top.down.is_opposite(p.top)
            }),
            (0, _) => find_matching_piece(pieces, |p| {
                let left = self.matrix[i][j - 1].unwrap();
                p.is_edge() && left.right.is_opposite(p.left)
            }),
            (M, N) => find_matching_piece(pieces, |p| {
                let left = self.matrix[i][j - 1].unwrap();
                let top = self.matrix[i - i][j].unwrap();
                left.right.is_opposite(p.left)
                    && top.down.is_opposite(p.top)
                    && p.down == Edge::Flat
                    && p.right == Edge::Flat
            }),
            (_, _) => find_matching_piece(pieces, |p| {
                let left = self.matrix[i][j - 1].unwrap();
                let top = self.matrix[i - i][j].unwrap();
                left.right.is_opposite(p.left) && top.down.is_opposite(p.top)
            }),
        }
    }
}

fn find_matching_piece<F>(pieces: &Vec<Piece>, f: F) -> (Piece, Piece)
where
    F: Fn(&Piece) -> bool,
{
    //assume we'll always find a matching piece
    let mut piece_at_right_angle = None;
    let mut to_remove = None;
    pieces.iter().find(|p| {
        let mut found = false;
        let mut piece = **p;
        to_remove = Some(piece.clone());
        for _ in 0..3 {
            if f(&piece) {
                found = true;
                piece_at_right_angle = Some(piece);
                break;
            } else {
                piece.rotate_right()
            }
        }

        found
    });

    (piece_at_right_angle.unwrap(), to_remove.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_puzzle() {
        let p0_0 = Piece {
            top: Edge::Flat,
            left: Edge::Flat,
            right: Edge::Inner,
            down: Edge::Outer,
            orientation: Orientation::Left,
        };

        let p0_1 = Piece {
            top: Edge::Flat,
            left: Edge::Outer,
            right: Edge::Inner,
            down: Edge::Outer,
            orientation: Orientation::Left,
        };

        let p0_2 = Piece {
            top: Edge::Flat,
            left: Edge::Outer,
            right: Edge::Flat,
            down: Edge::Outer,
            orientation: Orientation::Left,
        };

        let p1_0 = Piece {
            top: Edge::Inner,
            left: Edge::Flat,
            right: Edge::Inner,
            down: Edge::Outer,
            orientation: Orientation::Left,
        };

        let p1_1 = Piece {
            top: Edge::Inner,
            left: Edge::Outer,
            right: Edge::Inner,
            down: Edge::Outer,
            orientation: Orientation::Left,
        };

        let p1_2 = Piece {
            top: Edge::Inner,
            left: Edge::Outer,
            right: Edge::Flat,
            down: Edge::Outer,
            orientation: Orientation::Left,
        };

        let p2_0 = Piece {
            top: Edge::Inner,
            left: Edge::Flat,
            right: Edge::Outer,
            down: Edge::Flat,
            orientation: Orientation::Left,
        };

        let p2_1 = Piece {
            top: Edge::Inner,
            left: Edge::Inner,
            right: Edge::Outer,
            down: Edge::Flat,
            orientation: Orientation::Left,
        };

        let p2_2 = Piece {
            top: Edge::Inner,
            left: Edge::Inner,
            right: Edge::Flat,
            down: Edge::Flat,
            orientation: Orientation::Left,
        };

        let mut puzzle = Puzzle::new(
            M,
            N,
            vec![p0_0, p0_1, p1_1, p2_1, p2_2, p2_0, p1_2, p0_2, p1_0, p2_1],
        );
        puzzle.solve();
    }
}
