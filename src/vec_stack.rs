use std::fmt::{Debug, Display, Formatter};
use std::fmt;

#[derive(Debug)]
pub struct VecStack<T> {
    vec: Vec<T>
}

impl<T> VecStack<T>
    where T: Copy
{
    pub fn new() -> Self {
        VecStack {
            vec: vec![],
        }
    }

    pub fn push(&mut self, e: T) {
        self.vec.push(e);
    }

    pub fn peek(&mut self) -> Option<T> {
        let len = self.vec.len();
        match len {
            0 => None,
            _ => Some(self.vec[len - 1])
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }

    pub fn size(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.len() == 0
    }
}

#[allow(unused_must_use)]
impl<T: Display> Display for VecStack<T>
    where T: Debug
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_stack() {
        let mut stack = VecStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(3, stack.size());
        assert_eq!("[1, 2, 3]", format!("{}", stack));
    }

    #[test]
    fn should_peek_and_pop() {
        let mut stack = VecStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(Some(3), stack.peek());
        assert_eq!(Some(3), stack.pop());
        assert_eq!(Some(2), stack.peek());
    }

    #[test]
    fn should_be_empty() {
        let stack: VecStack<u32> = VecStack::new();
        assert!(stack.is_empty());
    }
}