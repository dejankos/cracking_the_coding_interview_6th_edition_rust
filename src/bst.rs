use std::cell::{RefCell, RefMut};
use std::fmt::{Debug, Display, Formatter};
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

type Link<T> = Rc<RefCell<Node<T>>>;

struct Tree<T> {
    root: Option<Link<T>>,
}

struct Node<T> {
    data: T,
    left: Option<Link<T>>,
    right: Option<Link<T>>,
}

impl<T> Tree<T>
    where
        T: PartialOrd + Debug + Copy,
{
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn insert(&mut self, data: T) {
        let node = Rc::new(RefCell::new(Node {
            data,
            left: None,
            right: None,
        }));

        if let Some(ref r) = self.root {
            let mut current = r.clone();

            loop {
                // left
                if node.borrow().data < current.clone().borrow().data {
                    if current.borrow().left.is_some() {
                        let l = current.borrow_mut().left.as_mut().unwrap().clone();
                        current = l;
                    } else {
                        current.borrow_mut().left = Some(node.clone());
                        break; // node inserted on the left
                    }
                }
                // right
                else {
                    if current.borrow().right.is_some() {
                        let r = current.borrow_mut().right.as_mut().unwrap().clone();
                        current = r;
                    } else {
                        current.borrow_mut().right = Some(node.clone());
                        break; // node inserted on the left
                    }
                }
            }
        } else {
            self.root = Some(node);
        }
    }

    pub fn in_order_traversal(&self) -> Vec<T> {
        let mut vec = vec![];
        if let Some(r) = &self.root {
            self.r_in_order_traversal(r, &mut vec);
        }

        vec
    }

    fn r_in_order_traversal(&self, node: &Link<T>, vec: &mut Vec<T>) {
        if let Some(left) = node.borrow().left.as_ref() {
            self.r_in_order_traversal(left, vec);
        }
        vec.push(node.borrow().data);
        if let Some(right) = node.borrow().right.as_ref() {
            self.r_in_order_traversal(right, vec);
        }
    }
}

#[allow(unused_must_use)]
impl<T: Display> Display for Tree<T>
    where T: PartialOrd + Debug + Copy,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.in_order_traversal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_build_a_bst() {
        let mut tree = Tree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(4);
        tree.insert(6);

        let res = tree.in_order_traversal();
        assert_eq!(vec![3, 4, 5, 6], res);
    }
}
