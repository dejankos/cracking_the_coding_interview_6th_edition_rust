//Return Kth to Last: Implement an algorithm to find the kth to last element of a singly linked list.

use crate::linked_list::{LinkedList, List, RcLink};

trait KtoLast<T>: List<T>
    where T: std::cmp::PartialEq + Clone
{
    fn k_to_last(&mut self, k: usize) -> Self;
}

impl<T> KtoLast<T> for LinkedList<T>
    where T: std::cmp::PartialEq + Clone
{
    fn k_to_last(&mut self, k: usize) -> Self {
        self.into_iter().skip(k) // since I've written Iterator for LL why not use it
            .collect::<Vec<RcLink<T>>>()
            .into_iter()
            .map(|rc| rc.borrow_mut().e.clone())
            .collect::<LinkedList<T>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::{LinkedList, List};

    use super::*;

    #[test]
    fn should_collect_into_new_list_from_kth_to_last() {
        let mut list = LinkedList::new();
        list.add(1);
        list.add(2);
        list.add(3);

        let k_to_last = list.k_to_last(1);

        //k to last
        assert_eq!(k_to_last.size(), 2);
        assert_eq!(format!("{}", k_to_last), "[2, 3]");

        //original list should remain unchanged
        assert_eq!(list.size(), 3);
        assert_eq!(format!("{}", list), "[1, 2, 3]");
    }
}