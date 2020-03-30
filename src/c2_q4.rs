// Partition: Write code to partition a linked list around a value x, such that all nodes less than x come
// before all nodes greater than or equal to x. If x is contained within the list, the values of x only need
// to be after the elements less than x (see below). The partition element x can appear anywhere in the
// "right partition"; it does not need to appear between the left and right partitions.
// EXAMPLE
// Input: 3 -> 5 -> 8 -> 5 -> 10 -> 2 -> 1 [partition= 5]
// Output: 3 -> 1 -> 2 -> 10 -> 5 -> 5 -> 8
// Hints: #3, #24

use crate::linked_list::{LinkedList, List};

trait Partition<T>: List<T>
    where T: PartialEq
{
    fn partition(&self, part_element: usize) -> Self;
}


impl<T> Partition<T> for LinkedList<T>
    where T: PartialEq
{
    fn partition(&self, part_element: usize) -> Self {
        LinkedList::new()
    }
}


#[cfg(test)]
mod tests {
    use crate::linked_list::{LinkedList, List};

    #[test]
    fn should_partition_list() {
        let mut list = LinkedList::new();
        list.add(3);
        list.add(5);
        list.add(8);
        list.add(5);
        list.add(10);
        list.add(2);
        list.add(1);


        assert_eq!(list.size(), 7);
    }
}