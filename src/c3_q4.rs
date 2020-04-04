// Implement a MyQueue class which implements a queue using two stacks.
// It uses the operations:
// add ( i tern): Add an item to the end of the list.
// remove (): Remove the first item in the list.
// peek ( ) : Return the top of the queue.
// is Empty(): Return true if and only if the queue is empty.

use crate::vec_stack::{Stack, VecStack};

struct Queue<T> {
    q: VecStack<T>,
    temp: VecStack<T>,
}

impl<T> Queue<T>
where
    T: Copy,
{
    fn new() -> Self {
        Queue {
            q: VecStack::new(),
            temp: VecStack::new(),
        }
    }

    fn add(&mut self, e: T) {
        self.q.push(e)
    }

    fn remove(&mut self) -> Option<T> {
        self.s_to_q_op(|v| v.pop())
    }

    fn peek(&mut self) -> Option<T> {
        self.s_to_q_op(|v| v.peek())
    }

    fn s_to_q_op<F>(&mut self, mut f: F) -> Option<T>
    where
        F: FnMut(&mut VecStack<T>) -> Option<T>,
    {
        self.drain_to_temp();
        let head = f(&mut self.temp);
        self.drain_to_q();

        head
    }

    fn is_empty(&self) -> bool {
        self.q.is_empty()
    }

    fn drain_to_temp(&mut self) {
        while let Some(v) = self.q.pop() {
            self.temp.push(v)
        }
    }

    fn drain_to_q(&mut self) {
        while let Some(v) = self.temp.pop() {
            self.q.push(v)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_add_and_remove_from_q() {
        let mut q = Queue::new();
        q.add(1);
        q.add(2);
        q.add(3);

        assert!(!q.is_empty());
        assert_eq!(1, q.remove().unwrap());
    }

    #[test]
    fn should_add_peek_and_remove_from_q() {
        let mut q = Queue::new();
        q.add(1);
        q.add(2);
        q.add(3);

        assert!(!q.is_empty());
        assert_eq!(1, q.peek().unwrap());
        assert_eq!(1, q.remove().unwrap());
    }
}
