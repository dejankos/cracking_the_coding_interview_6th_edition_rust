// BST Sequences: A binary search tree was created by traversing through an array from left to right
// and inserting each element. Given a binary search tree with distinct elements, print all possible
// arrays that could have led to this tree.

use itertools::Itertools;

use crate::bst::{Link, Tree};
use crate::c4_q3::build_depths;

fn possible_combinations(tree: Tree<usize>) -> Vec<Vec<usize>> {
    let mut lvl_per = build_depths(tree)
        .iter()
        .map(|lvl| lvl_per(lvl.as_ref().unwrap()))
        .collect::<Vec<Vec<Vec<usize>>>>();

    let mut lvl_idx = 1;
    let mut res = vec![lvl_per[0][0].clone()];

    // iter lvl per lvl while joining all combinations
    while lvl_idx <= lvl_per.len() - 1 {
        let lvl = &lvl_per[lvl_idx];
        let mut lvl_join = vec![];
        for v in res.iter() {
            for l in lvl.iter() {
                let joined = v
                    .iter()
                    .copied()
                    .chain(l.iter().copied())
                    .collect::<Vec<usize>>();
                lvl_join.push(joined);
            }
        }
        lvl_idx = lvl_idx + 1;
        res = lvl_join;
    }
    res
}

fn lvl_per(lvl: &Vec<usize>) -> Vec<Vec<usize>> {
    lvl.iter()
        .permutations(lvl.len())
        .unique()
        .map(|v| v.into_iter().map(|r| *r).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::c4_q3::build_depths;

    use super::*;

    #[test]
    fn should_compute_all_combinations() {
        let mut tree = Tree::new();
        tree.insert(2);
        tree.insert(1);
        tree.insert(3);

        let res = possible_combinations(tree);
        assert_eq!(vec![2, 1, 3], res[0]);
        assert_eq!(vec![2, 3, 1], res[1]);
    }

    #[test]
    fn should_compute_all_combinations_1() {
        let mut tree = Tree::new();
        tree.insert(2);
        tree.insert(1);
        tree.insert(4);
        tree.insert(3);
        tree.insert(5);

        let res = possible_combinations(tree);
        assert_eq!(vec![2, 1, 4, 3, 5], res[0]);
        assert_eq!(vec![2, 1, 4, 5, 3], res[1]);
        assert_eq!(vec![2, 4, 1, 3, 5], res[2]);
        assert_eq!(vec![2, 4, 1, 5, 3], res[3]);
    }
}
