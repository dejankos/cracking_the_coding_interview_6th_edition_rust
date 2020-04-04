//Stack Min: How would you design a stack which, in addition to push and pop, has a function min
//which returns the minimum element? Push, pop and min should all operate in 0(1) time.

use std::cmp::min;
use std::f32::MAX;

use crate::vec_stack::{Stack, VecStack};

#[derive(Debug)]
struct Min {
    min: VecStack<usize>,
    stack: VecStack<usize>,
}

impl Min {
    fn curr_min(&mut self) -> usize {
        match self.min.peek() {
            Some(v) => v,
            _ => usize::max_value(),
        }
    }
}

impl Stack<usize> for Min {
    fn new() -> Self {
        Min {
            min: VecStack::new(),
            stack: VecStack::new(),
        }
    }

    fn push(&mut self, e: usize) {
        if e < self.curr_min() {
            self.min.push(e)
        }

        self.stack.push(e);
    }

    fn peek(&mut self) -> Option<usize> {
        self.stack.peek()
    }

    fn pop(&mut self) -> Option<usize> {
        let v = self.stack.pop();
        if v.is_some() && v.unwrap() == self.curr_min() {
            self.min.pop();
        }

        v
    }

    fn size(&self) -> usize {
        self.stack.size()
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_yield_min() {
        let mut min = Min::new();
        min.push(5);
        min.push(7);
        min.push(3);
        min.push(10);

        assert_eq!(3, min.curr_min());
        assert_eq!(10, min.pop().unwrap());
        assert_eq!(3, min.curr_min());
        assert_eq!(3, min.pop().unwrap());
        assert_eq!(5, min.curr_min());
        assert_eq!(7, min.pop().unwrap());
        assert_eq!(5, min.curr_min());
    }
}
