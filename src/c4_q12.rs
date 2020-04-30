// Paths with Sum: You are given a binary tree in which each node contains an integer value (which
// might be positive or negative). Design an algorithm to count the number of paths that sum to a
// given value. The path does not need to start or end at the root or a leaf, but it must go downwards
// (traveling only from parent nodes to child nodes).

use crate::binary_tree::{Link, Tree};

fn find_sum_path(tree: Tree<isize>, sum: isize) -> Vec<isize> {
    search_path(tree.root.as_ref().unwrap(), sum)
}

fn search_path(node: &Link<isize>, sum: isize) -> Vec<isize> {
    let mut path: Vec<isize> = vec![];
    let found = find_from_node(node, sum, 0, &mut path.clone(), &mut path);
    return if found {
        path
    } else {
        let (mut l, mut r) = (vec![], vec![]);
        if let Some(ref left) = node.borrow().left {
            l = search_path(left, sum);
        }

        if let Some(ref right) = node.borrow().right {
            r = search_path(right, sum);
        }

        if !l.is_empty() {
            l
        } else {
            r
        }
    };
}

fn find_from_node(
    node: &Link<isize>,
    sum: isize,
    current: isize,
    path: &mut Vec<isize>,
    found_path: &mut Vec<isize>,
) -> bool {
    let c_sum = current + node.borrow().data;
    path.push(node.borrow().data);
    return if c_sum == sum {
        found_path.append(path);
        true
    } else if c_sum > sum {
        false
    } else {
        let (mut l, mut r) = (false, false);
        if let Some(ref left) = node.borrow().left {
            let mut l_path = path.clone();
            l = find_from_node(left, sum, c_sum, &mut l_path, found_path);
        }

        if let Some(ref right) = node.borrow().right {
            let mut r_path = path.clone();
            r = find_from_node(right, sum, c_sum, &mut r_path, found_path);
        }

        l || r
    };
}

#[cfg(test)]
mod tests {
    use crate::binary_tree::Tree;

    use super::*;

    #[test]
    fn should_find_a_search_path_from_root() {
        let mut tree = Tree::new();
        tree.insert(2);
        tree.insert(1);
        tree.insert(4);
        tree.insert(3);
        tree.insert(6);

        let res = find_sum_path(tree, 12);
        assert_eq!(vec![2, 4, 6], res);
    }

    #[test]
    fn should_find_a_search_path_from_subtree_root() {
        let mut tree = Tree::new();
        tree.insert(2);
        tree.insert(1);
        tree.insert(4);
        tree.insert(3);
        tree.insert(6);

        let res = find_sum_path(tree, 10);
        assert_eq!(vec![4, 6], res);
    }

    #[test]
    fn should_find_a_search_path_from_subtree_root_1() {
        let mut tree = Tree::new();
        tree.insert(2);
        tree.insert(1);
        tree.insert(4);
        tree.insert(3);
        tree.insert(6);

        let res = find_sum_path(tree, 7);
        assert_eq!(vec![4, 3], res);
    }

    #[test]
    fn should_not_find_a_search_path() {
        let mut tree = Tree::new();
        tree.insert(2);
        tree.insert(1);
        tree.insert(4);
        tree.insert(3);
        tree.insert(6);

        let res = find_sum_path(tree, 42);
        assert!(res.is_empty());
    }
}
