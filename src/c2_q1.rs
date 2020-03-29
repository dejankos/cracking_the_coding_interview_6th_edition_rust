// Remove Dups! Write code to remove duplicates from an unsorted linked list.
// How would you solve this problem if a temporary buffer is not allowed?

use crate::linked_list::{List, LinkedList};
use std::collections::HashSet;
use std::hash::Hash;

trait Duplicates<T>: List<T>
    where T: PartialEq + Eq + Hash + Copy
{
    fn remove_duplicates(&mut self);
}

impl<T> Duplicates<T> for LinkedList<T>
    where T: PartialEq + Eq + Hash + Copy
{
    fn remove_duplicates(&mut self) {
        let mut h_set = HashSet::new();
        for node in self.into_iter() {
            let value = node.borrow_mut().e.clone();

            if h_set.contains(&value) {
                node.borrow_mut().unlink(self);
            } else {
                h_set.insert(value);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_remove_duplicates() {
        let mut list:LinkedList<u32>  = List::new();
        list.add(1);
        list.add(2);
        list.add(1);
        list.add(3);
        list.add(4);
        list.add(3);

        list.remove_duplicates();

        assert_eq!(format!("{}", list), "[1, 2, 3, 4]");
    }
}