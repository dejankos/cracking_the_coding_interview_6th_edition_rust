// Check Subtree: Tl and T2 are two very large binary trees, with Tl much bigger than T2. Create an
// algorithm to determine if T2 is a subtree of Tl.
// A tree T2 is a subtree of Tl if there exists a node n in Tl such that the subtree of n is identical to T2.
// That is, if you cut off the tree at node n, the two trees would be identical.

#[cfg(test)]
mod tests {
    use crate::binary_tree::Tree;

    use super::*;

    #[test]
    fn should_be_a_subtree() {
        let mut tree = Tree::new();
        tree.insert(2);
        tree.insert(1);
        tree.insert(4);
        tree.insert(3);
        tree.insert(5);

        let mut sub_tree = Tree::new();
        sub_tree.insert(4);
        sub_tree.insert(3);
        sub_tree.insert(5);

        assert!(tree.is_subtree(sub_tree));
    }

    #[test]
    fn should_not_be_a_subtree() {
        let mut tree = Tree::new();
        tree.insert(2);
        tree.insert(1);
        tree.insert(4);
        tree.insert(3);
        tree.insert(5);

        let mut other_tree = Tree::new();
        other_tree.insert(4);
        other_tree.insert(3);
        other_tree.insert(6);

        assert!(!tree.is_subtree(other_tree));
    }

    #[test]
    fn should_not_be_a_subtree_1() {
        let mut tree = Tree::new();
        tree.insert(2);
        tree.insert(1);
        tree.insert(4);
        tree.insert(3);
        tree.insert(5);

        let mut other_tree = Tree::new();
        other_tree.insert(4);

        assert!(!tree.is_subtree(other_tree));
    }

    #[test]
    fn should_not_be_a_subtree_2() {
        let mut tree = Tree::new();
        tree.insert(2);
        tree.insert(1);
        tree.insert(4);
        tree.insert(3);
        tree.insert(5);

        let mut other_empty_tree = Tree::new();
        assert!(!tree.is_subtree(other_empty_tree));
    }

    #[test]
    fn should_not_be_a_subtree_3() {
        let mut tree = Tree::new();
        tree.insert(2);
        tree.insert(1);
        tree.insert(4);
        tree.insert(3);
        tree.insert(5);

        let mut other_tree = Tree::new();
        other_tree.insert(42);

        assert!(!tree.is_subtree(other_tree));
    }
}
