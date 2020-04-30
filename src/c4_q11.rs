// Random Node: You are implementing a binary tree class from scratch which, in addition to
// insert, find, and delete, has a method getRandomNode() which returns a random node
// from the tree. All nodes should be equally likely to be chosen. Design and implement an algorithm
// for getRandomNode, and explain how you would implement the rest of the methods.

use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::binary_tree::Tree;
use crate::c4_q3::build_depths;

fn get_random_node(tree: Tree<usize>) -> usize {
    let current_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as usize;
    let depths = build_depths(tree);
    let inner = depths[current_ms % depths.len()].as_ref().unwrap();

    inner[current_ms % inner.len()]
}

#[cfg(test)]
mod tests {
    use crate::binary_tree::Tree;

    use super::*;

    #[test]
    fn should_read_a_random_node_val() {
        let mut tree = Tree::new();
        tree.insert(2);
        tree.insert(1);
        tree.insert(4);
        tree.insert(3);
        tree.insert(5);

        let r = get_random_node(tree);
        assert!(r >= 1 && r <= 5);
    }
}
