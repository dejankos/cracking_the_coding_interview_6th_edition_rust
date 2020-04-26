// List of Depths: Given a binary tree, design an algorithm which creates a linked list of all the nodes
// at each depth (e.g., if you have a tree with depth D, you'll have D linked lists).

use crate::bst::{Link, Tree};

pub fn build_depths(tree: Tree<usize>) -> Vec<Option<Vec<usize>>> {
    let mut vec = vec![];
    if let Some(r) = &tree.root {
        r_pre_order_traversal(r, &mut vec, 0);
        vec
    } else {
        vec
    }
}

fn r_pre_order_traversal(
    node: &Link<usize>,
    depth_vec: &mut Vec<Option<Vec<usize>>>,
    depth: usize,
) {
    if depth == depth_vec.len() {
        depth_vec.push(Some(vec![node.borrow().data]))
    } else if let Some(v) = depth_vec[depth].as_mut() {
        v.push(node.borrow().data);
    }

    if let Some(ref left) = node.borrow().left {
        r_pre_order_traversal(left, depth_vec, depth + 1);
    }
    if let Some(ref right) = node.borrow().right {
        r_pre_order_traversal(right, depth_vec, depth + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_build_a_list_per_depth() {
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

        let mut depths = build_depths(tree);
        assert_eq!(vec![5], *depths[0].as_ref().unwrap());
        assert_eq!(vec![2, 7], *depths[1].as_ref().unwrap());
        assert_eq!(vec![1, 3, 6, 8], *depths[2].as_ref().unwrap());
        assert_eq!(vec![4, 9], *depths[3].as_ref().unwrap());
        assert_eq!(None, depths.get(4));
    }
}
