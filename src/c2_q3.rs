// Delete Middle Node: Implement an algorithm to delete a node in the middle (i.e., any node but
// the first and last node, not necessarily the exact middle) of a singly linked list, given only access to
// that node.
// EXAMPLE
// Input:the node c from the linked list a->b->c->d->e->f
// Result: nothing is returned, but the new linked list looks like a->b->d->e- >f

// Got this one for free as I've implemented it in the base Linked List implementation
// check remove impl at  src/linked_list.rs:19

#[cfg(test)]
mod tests {
    use crate::linked_list::{LinkedList, List};

    #[test]
    fn should_remove_some_element_from_the_middle() {
        let mut list = LinkedList::new();
        list.add('a');
        list.add('b');
        list.add('c');
        list.add('d');
        list.add('e');
        list.add('f');

        list.remove('c');

        assert_eq!(list.size(), 5);
        assert_eq!(format!("{}", list), "[a, b, d, e, f]");
    }
}
