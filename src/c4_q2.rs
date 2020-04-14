// Minimal Tree: Given a sorted (increasing order) array with unique integer elements, write an algo­rithm
// to create a binary search tree with minimal height.

use crate::bst::Tree;

fn build_min_height(s_arr: &[usize]) -> Tree<usize> {
    let mut tree = Tree::new();
    r_build_min_height(&s_arr, 0, s_arr.len() - 1, &mut tree);

    tree
}

fn r_build_min_height(s_arr: &[usize], start: usize, end: usize, tree: &mut Tree<usize>) {
    if end < start {
        return;
    }

    let mid = (start + end) / 2;
    tree.insert(s_arr[mid]);

    if mid > 0 {
        r_build_min_height(&s_arr, start, mid - 1, tree);
    }
    r_build_min_height(&s_arr, mid + 1, end, tree);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_build_a_bst_with_min_height() {
        let s_arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];

        let tree = build_min_height(&s_arr);
        assert_eq!(s_arr.to_vec(), tree.in_order_traversal());
    }
}
