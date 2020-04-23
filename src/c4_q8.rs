// First Common Ancestor: Design an algorithm and write code to find the first common ancestor
// of two nodes in a binary tree. Avoid storing additional nodes in a data structure. NOTE: This is not
// necessarily a binary search tree.


use std::fs::read;
use std::rc::Rc;

use crate::bst::{Link, Tree};

fn find_first_common_ancestor(tree: Tree<usize>, fn_val: usize, sn_val: usize) -> usize {
    r_find(&tree, tree.root.as_ref().unwrap(), &tree.find_node(fn_val).unwrap(), &tree.find_node(sn_val).unwrap()).
        borrow().
        data
}

fn r_find(tree: &Tree<usize>, current: &Link<usize>, f: &Link<usize>, s: &Link<usize>) -> Link<usize> {
    println!("current {}", current.borrow().data);

    if Rc::ptr_eq(current, f) || Rc::ptr_eq(current, s) {
        println!("found node by ref {}", current.borrow().data);
        return current.clone();
    }

    let mut left = (false, true);
    if let Some(ref v) = current.borrow().left {
        left = is_left(tree, Some(f), Some(s), v);
    }
    if left.0 != left.1 {
        println!("found node diverge {}", current.borrow().data);
        return current.clone();
    }

    return if left.1 {
        println!("moving left");
        r_find(tree, current.borrow().left.as_ref().unwrap(), f, s)
    } else {
        println!("moving right");
        r_find(tree, current.borrow().right.as_ref().unwrap(), f, s)
    };
}

fn is_left(tree: &Tree<usize>, f: Option<&Link<usize>>, s: Option<&Link<usize>>, left: &Link<usize>) -> (bool, bool) {
    let (mut fl, mut sl) = (false, false);
    if let Some(f_rc) = f {
        let found = tree.r_find_node(f_rc.borrow().data, left);
        fl = found.is_some();
    }

    if let Some(s_rc) = s {
        let found = tree.r_find_node(s_rc.borrow().data, left);
        sl = found.is_some();
    }

    (fl, sl)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_first_common_ancestor_in_bst() {
        let mut tree = Tree::new();
        tree.insert(5);
        tree.insert(2);
        tree.insert(7);
        tree.insert(1);
        tree.insert(3);

        let res = find_first_common_ancestor(tree, 1, 3);
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

        let res = find_first_common_ancestor(tree, 1, 7);
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

        let res = find_first_common_ancestor(tree, 2, 3);
        assert_eq!(2, res)
    }
}