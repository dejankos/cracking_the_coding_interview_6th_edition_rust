// First Common Ancestor: Design an algorithm and write code to find the first common ancestor
// of two nodes in a binary tree. Avoid storing additional nodes in a data structure. NOTE: This is not
// necessarily a binary search tree.

use std::rc::Rc;

use crate::binary_tree::{Link, Tree};

fn find_first_common_ancestor_bst(tree: Tree<usize>, fn_val: usize, sn_val: usize) -> usize {
    let f = &tree.find_node_bt(fn_val).unwrap();
    let s = &tree.find_node_bt(sn_val).unwrap();

    find_first_common_ancestor_bt(tree, f, s)
}

fn find_first_common_ancestor_bt(tree: Tree<usize>, f: &Link<usize>, s: &Link<usize>) -> usize {
    r_find(&tree, tree.root.as_ref().unwrap(), f, s)
        .borrow()
        .data
}

fn r_find(
    tree: &Tree<usize>,
    current: &Link<usize>,
    f: &Link<usize>,
    s: &Link<usize>,
) -> Link<usize> {
    if Rc::ptr_eq(current, f) || Rc::ptr_eq(current, s) {
        return current.clone();
    }

    let mut left = (false, true);
    if let Some(ref v) = current.borrow().left {
        left = is_left(tree, Some(f), Some(s), v);
    }
    if left.0 != left.1 {
        return current.clone();
    }

    return if left.1 {
        r_find(tree, current.borrow().left.as_ref().unwrap(), f, s)
    } else {
        r_find(tree, current.borrow().right.as_ref().unwrap(), f, s)
    };
}

fn is_left(
    tree: &Tree<usize>,
    f: Option<&Link<usize>>,
    s: Option<&Link<usize>>,
    left: &Link<usize>,
) -> (bool, bool) {
    let (mut fl, mut sl) = (false, false);
    if let Some(f_rc) = f {
        fl = tree.r_contains_node_bt(f_rc.borrow().data, left);
    }

    if let Some(s_rc) = s {
        sl = tree.r_contains_node_bt(s_rc.borrow().data, left);
    }

    (fl, sl)
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::binary_tree::Node;

    use super::*;

    #[test]
    fn should_find_first_common_ancestor_in_bst() {
        let mut tree = Tree::new();
        tree.insert(5);
        tree.insert(2);
        tree.insert(7);
        tree.insert(1);
        tree.insert(3);

        let res = find_first_common_ancestor_bst(tree, 1, 3);
        assert_eq!(2, res)
    }

    #[test]
    fn should_find_root_as_first_common_ancestor_in_bst() {
        let mut tree = Tree::new();
        tree.insert(5);
        tree.insert(2);
        tree.insert(7);
        tree.insert(1);
        tree.insert(3);

        let res = find_first_common_ancestor_bst(tree, 1, 7);
        assert_eq!(5, res)
    }

    #[test]
    fn should_find_subtree_root_as_first_common_ancestor_in_bst() {
        let mut tree = Tree::new();
        tree.insert(5);
        tree.insert(2);
        tree.insert(7);
        tree.insert(1);
        tree.insert(3);

        let res = find_first_common_ancestor_bst(tree, 2, 3);
        assert_eq!(2, res)
    }

    #[test]
    fn should_find_first_common_ancestor_in_bt() {
        let left_l1 = Rc::new(RefCell::new(Node {
            data: 45,
            left: None,
            right: None,
        }));
        let left_r1 = Rc::new(RefCell::new(Node {
            data: 42,
            left: None,
            right: None,
        }));
        let right_l1 = Rc::new(RefCell::new(Node {
            data: 100,
            left: None,
            right: None,
        }));
        let left = Rc::new(RefCell::new(Node {
            data: 20,
            left: Some(left_l1.clone()),
            right: Some(left_r1.clone()),
        }));
        let right = Rc::new(RefCell::new(Node {
            data: 10,
            left: Some(right_l1.clone()),
            right: None,
        }));
        let root = Rc::new(RefCell::new(Node {
            data: 5,
            left: Some(left.clone()),
            right: Some(right),
        }));

        let mut tree = Tree::new();
        tree.build_from(root);

        let res = find_first_common_ancestor_bt(tree, &left_l1.clone(), &left_r1.clone());
        assert_eq!(20, res)
    }
}
