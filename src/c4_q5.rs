// Validate BST: Implement a function to check if a binary tree is a binary search tree.

use crate::bst::Tree;

fn is_bst(tree: Tree<usize>) -> bool {
    let vec = tree.in_order_traversal();
    for (idx, val) in vec.iter().enumerate().skip(1) {
        if val < &vec[idx - 1] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::bst::Node;

    use super::*;

    #[test]
    fn should_be_bst() {
        let mut tree = Tree::new();
        tree.insert(5);
        tree.insert(2);
        tree.insert(7);
        tree.insert(1);
        tree.insert(3);

        assert!(is_bst(tree))
    }

    #[test]
    fn should_not_be_bst() {
        let left = Rc::new(RefCell::new(Node {
            data: 20,
            left: None,
            right: None,
        }));
        let right = Rc::new(RefCell::new(Node {
            data: 10,
            left: None,
            right: None,
        }));
        let root = Rc::new(RefCell::new(Node {
            data: 5,
            left: Some(left),
            right: Some(right),
        }));

        let mut tree = Tree::new();
        tree.build_from(root);

        assert!(!is_bst(tree))
    }
}
