// A basic linked list implementation for all chapter 2 problems
//
// Implementation based on resources ->
// https://doc.rust-lang.org/1.30.0/book/second-edition/ch15-05-interior-mutability.html#having-multiple-owners-of-mutable-data-by-combining-rct-and-refcellt
// https://rust-unofficial.github.io/too-many-lists/fourth-final.html

use std::cell::RefCell;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    e: T,
    next: Link<T>,
    prev: Link<T>,
}

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
    size: u32,
}

pub struct IntoIter<T> {
    next: Link<T>
}

impl<T> Node<T> {
    fn new(e: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            e,
            prev: None,
            next: None,
        }))
    }

    fn unlink(&mut self) {
        match self.prev.take() {
            Some(prev) => {
                prev.borrow_mut().next = if let Some(next) = self.next.take() {
                    Some(next)
                } else {
                    None
                };
            }
            _ => {}
        }
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None, tail: None, size: 0 }
    }

    pub fn add(&mut self, e: T) {
        let new_node = Node::new(e);
        match self.tail.take() {
            Some(tail) => {
                tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(tail);

                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
        self.size = self.size + 1;
    }

    pub fn remove(&mut self, e: T) {}

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter { next: self.head.clone() }
    }
}

impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut current = self.head.clone();
        write!(f, "{}", "[");
        while current.is_some() {
            write!(f, "{}", current.clone().unwrap().borrow().e);
            current = current.unwrap().borrow_mut().next.clone();

            // current is next here - print element delimiter
            if current.is_some() {
                write!(f, "{}", ", ");
            }
        }
        write!(f, "{}", "]")
    }
}

// impl<T> Iterator for IntoIter<T> {
//     type Item = Link<T>;
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.next.take() {
//             Some(current) => {
//                 self.next = current.borrow_mut().next.clone();
//                 Some(Some(current))
//             }
//             _ => None
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_correct_size() {
        let mut list: List<u32> = List::new();
        list.add(1);
        list.add(2);
        list.add(3);

        assert_eq!(list.size, 3);
    }

    #[test]
    fn should_format_list() {
        let mut list: List<u32> = List::new();
        list.add(1);
        list.add(2);
        list.add(3);

        assert_eq!(format!("{}", list), "[1, 2, 3]");
    }
}

fn main() {
    let mut list: List<u32> = List::new();
    list.add(1);
    list.add(2);
    list.add(3);

    println!("{}", list);
}