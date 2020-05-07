// Successor: Write an algorithm to find the "next" node (i.e., in-order successor) of a given node in a
// binary search tree. You may assume that each node has a link to its parent.

use crate::binary_tree::Tree;

fn find_in_order_successor(tree: Tree<usize>, n_val: usize) -> Option<usize> {
    if let Some(n) = tree.find_node(n_val) {
        return if let Some(ref right) = n.borrow().right {
            tree.min_from(right)
        } else {
            let mut root = tree.root();
            let mut temp = None;

            while root.as_ref().is_some() {
                if n_val < root.as_ref().unwrap().borrow().data {
                    temp = root.clone();
                    root = root.take().unwrap().borrow().left.clone();
                } else if n_val > root.as_ref().unwrap().borrow().data {
                    root = root.take().unwrap().borrow().right.clone();
                } else {
                    break;
                }
            }
            if let Some(link) = temp {
                Some(link.borrow().data)
            } else {
                None
            }
        };
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_in_order_successor_in_right_subtree() {
        let mut tree = Tree::new();
        tree.insert(5);
        tree.insert(2);
        tree.insert(7);
        tree.insert(1);
        tree.insert(3);

        let res = find_in_order_successor(tree, 2);

        assert!(res.is_some());
        assert_eq!(res.unwrap(), 3);
    }

    #[test]
    fn should_find_in_order_successor_as_root() {
        let mut tree = Tree::new();
        tree.insert(5);
        tree.insert(2);
        tree.insert(7);
        tree.insert(1);
        tree.insert(3);

        let res = find_in_order_successor(tree, 3);

        assert!(res.is_some());
        assert_eq!(res.unwrap(), 5);
    }

    #[test]
    fn should_find_root_in_order_successor_in_right_subtree() {
        let mut tree = Tree::new();
        tree.insert(5);
        tree.insert(2);
        tree.insert(7);
        tree.insert(1);
        tree.insert(3);

        let res = find_in_order_successor(tree, 5);

        assert!(res.is_some());
        assert_eq!(res.unwrap(), 7);
    }

    #[test]
    fn should_not_find_in_order_successor() {
        let mut tree = Tree::new();
        tree.insert(5);
        tree.insert(2);
        tree.insert(7);
        tree.insert(1);
        tree.insert(3);

        let res = find_in_order_successor(tree, 7);

        assert!(res.is_none());
    }
}
