// Check Balanced: Implement a function to check if a binary tree is balanced. For the purposes of
// this question, a balanced tree is defined to be a tree such that the heights of the two subtrees of any
// node never differ by more than one.

use crate::binary_tree::Tree;

fn is_balanced(tree: Tree<usize>) -> bool {
    if let Some(r) = &tree.root {
        let (mut lh, mut rh) = (0, 0);
        if let Some(ref left) = r.borrow().left {
            lh = tree.r_height(left) as isize;
        }
        if let Some(ref right) = r.borrow().right {
            rh = tree.r_height(right) as isize;
        }

        (rh - lh).abs() <= 1
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_balanced() {
        let mut tree = Tree::new();
        tree.insert(5);
        tree.insert(2);
        tree.insert(7);
        tree.insert(1);
        tree.insert(3);
        tree.insert(6);
        tree.insert(8);
        tree.insert(4);
        tree.insert(9);

        assert!(is_balanced(tree))
    }

    #[test]
    fn should_not_be_balanced() {
        let mut tree = Tree::new();
        tree.insert(1);
        tree.insert(2);
        tree.insert(3);
        tree.insert(4);

        assert!(!is_balanced(tree))
    }
}
