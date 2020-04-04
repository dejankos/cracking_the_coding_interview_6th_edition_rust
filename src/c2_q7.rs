// Intersection: Given two (singly) linked lists, determine if the two lists intersect. Return the inter­
// secting node. Note that the intersection is defined based on reference, not value. That is, if the kth
// node of the first linked list is the exact same node (by reference) as the jth node of the second
// linked list, then they are intersecting.

use std::rc::Rc;

use crate::linked_list::{Link, LinkedList, List};

fn intersection_node(list1: &mut LinkedList<u32>, list2: &mut LinkedList<u32>) -> Link<u32> {
    for (l, r) in list1.into_iter().zip(list2.into_iter()) {
        if Rc::ptr_eq(&l, &r) {
            return Some(l);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::linked_list::{LinkedList, List, Node, SinglyLinkedReferenceExtension};

    use super::*;

    #[test]
    fn should_intersect() {
        let node = Rc::new(RefCell::new(Node {
            e: 100,
            prev: None,
            next: None,
        }));

        let (mut list1, mut list2): (LinkedList<u32>, LinkedList<u32>) =
            (LinkedList::new(), LinkedList::new());
        list1.add(1);
        list1.add(2);
        list1.add(1);
        list1.add_node(Some(node.clone()));

        list2.add(1);
        list2.add(2);
        list2.add(1);
        list2.add_node(Some(node.clone()));

        //https://doc.rust-lang.org/std/rc/struct.Rc.html#method.ptr_eq
        assert!(Rc::ptr_eq(&node.clone(), &node.clone()));

        assert_eq!(
            100,
            intersection_node(&mut list1, &mut list2)
                .unwrap()
                .borrow_mut()
                .e
        )
    }

    #[test]
    fn should_not_intersect() {
        let node = Rc::new(RefCell::new(Node {
            e: 100,
            prev: None,
            next: None,
        }));

        let other_node = Rc::new(RefCell::new(Node {
            e: 100,
            prev: None,
            next: None,
        }));

        let (mut list1, mut list2): (LinkedList<u32>, LinkedList<u32>) =
            (LinkedList::new(), LinkedList::new());
        list1.add(1);
        list1.add(2);
        list1.add(1);
        list1.add_node(Some(node.clone()));

        list2.add(1);
        list2.add(2);
        list2.add(1);
        list2.add_node(Some(other_node.clone()));

        assert!(!Rc::ptr_eq(&node.clone(), &other_node.clone()));
        assert!(intersection_node(&mut list1, &mut list2).is_none())
    }
}
