// Loop Detection: Given a circular linked list, implement an algorithm that returns the node at the
// beginning of the loop.
// DEFINITION
// Circular linked list: A (corrupt) linked list in which a node's next pointer points to an earlier node, so
// as to make a loop in the linked list.
// EXAMPLE
// Input:
// Output:
// A -> B -> C -> D -> E -> C [the same C as earlier]
// C

use std::fmt::Display;
use std::rc::Rc;

use crate::linked_list::{Link, LinkedList, List};

trait LoopDetection<T>: List<T>
where
    T: PartialEq,
{
    fn find_loop_node(&mut self) -> Link<T>;
}

impl<T> LoopDetection<T> for LinkedList<T>
where
    T: PartialEq + Display,
{
    fn find_loop_node(&mut self) -> Link<T> {
        // fast and slow aka tortoise and hare
        let (s_iter, f_iter) = (self.into_iter(), self.into_iter().skip(3).step_by(2));

        for (s, f) in s_iter.zip(f_iter) {
            if Rc::ptr_eq(&s, &f) {
                // we have a match but it can be any node in the loop
                let (mut s_iter, mut f_iter) = (self.into_iter(), self.into_iter());
                let mut f_current = f_iter.next().unwrap();
                // not optimal but less painful because Rust - adjust fast iterator to matched position
                // but advance only at one step to find root of loop
                loop {
                    if Rc::ptr_eq(&s, &f_current) {
                        break;
                    }

                    f_current = f_iter.next().unwrap();
                }

                let mut s_current = s_iter.next().unwrap();
                while !Rc::ptr_eq(&s_current, &f_current) {
                    f_current = f_iter.next().unwrap();
                    s_current = s_iter.next().unwrap();
                }

                return Some(s_current);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {

    use crate::linked_list::{LinkedList, List, Node, SinglyLinkedReferenceExtension};

    use super::*;

    #[test]
    fn should_be_circular() {
        let c_node = Node::new('C');
        let circular_linked_nodes = Node::new_('D', Some(Node::new_('E', Some(c_node.clone()))));
        c_node.borrow_mut().next = Some(circular_linked_nodes);

        let mut list = LinkedList::new();
        list.add('A');
        list.add('B');

        list.add_node(Some(c_node));

        assert_eq!('C', list.find_loop_node().unwrap().borrow_mut().e);
    }

    #[test]
    fn should_not_be_circular() {
        let mut list = LinkedList::new();
        list.add('A');
        list.add('B');
        list.add('C');

        assert!(list.find_loop_node().is_none());
    }
}
