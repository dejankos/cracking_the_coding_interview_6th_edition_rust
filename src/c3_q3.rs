// Stack of Plates: Imagine a (literal) stack of plates. If the stack gets too high, it might topple.
// Therefore, in real life, we would likely start a new stack when the previous stack exceeds some
// threshold. Implement a data structure SetOfStacks that mimics this. SetO-fStacks should be
// composed of several stacks and should create a new stack once the previous one exceeds capacity.
// SetOfStacks. push() and SetOfStacks. pop() should behave identically to a single stack
// (that is, pop () should return the same values as it would if there were just a single stack).
// FOLLOW UP
// Implement a function popAt ( int index) which performs a pop operation on a specific sub-stack.

use crate::vec_stack::{Stack, VecStack};

#[derive(Debug)]
struct SetOfStacks<T> {
    stacks: Vec<VecStack<T>>,
    a_reg: AddressRegister,
}

#[derive(Default, Debug)]
struct AddressRegister {
    max: usize,
    current: usize,
}

impl<T> SetOfStacks<T>
where
    T: Copy,
{
    fn new(max: usize) -> Self {
        SetOfStacks {
            stacks: vec![VecStack::new()],
            a_reg: AddressRegister {
                max,
                ..Default::default()
            },
        }
    }

    fn is_limit_reached(&self) -> bool {
        self.stacks[self.a_reg.current].size() >= self.a_reg.max
    }

    fn switch_stack(&mut self) {
        self.a_reg.current = self.a_reg.current + 1;
        self.stacks.push(VecStack::new());
    }

    fn stack(&mut self) -> &mut VecStack<T> {
        &mut self.stacks[self.a_reg.current]
    }

    fn check_and_switch(&mut self) {
        if self.is_limit_reached() {
            self.switch_stack()
        }
    }

    fn n_of_stacks(&self) -> usize {
        self.stacks.len()
    }

    // FOLLOW UP
    fn pop_at(&mut self, stack_idx: usize) -> Option<T> {
        if stack_idx > self.a_reg.current {
            panic!("Stack index out of bounds")
        }

        self.stacks[stack_idx].pop()
    }
}

impl<T> Stack<T> for SetOfStacks<T>
where
    T: Copy,
{
    fn new() -> Self {
        unimplemented!("Unbounded stack not supported!")
    }

    fn push(&mut self, e: T) {
        self.check_and_switch();
        self.stack().push(e);
    }

    fn peek(&mut self) -> Option<T> {
        self.stack().peek()
    }

    fn pop(&mut self) -> Option<T> {
        self.stack().pop()
    }

    fn size(&self) -> usize {
        let as_ref = &self.stacks;
        as_ref.into_iter().map(|s| s.size()).sum()
    }

    fn is_empty(&self) -> bool {
        self.stacks[0].is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_switch_underlying_stacks() {
        let mut stack_set = SetOfStacks::new(2);
        stack_set.push(1);
        stack_set.push(2);

        assert_eq!(1, stack_set.n_of_stacks());

        stack_set.push(3);
        stack_set.push(4);

        assert_eq!(2, stack_set.n_of_stacks());

        stack_set.push(5);
        stack_set.push(6);

        assert_eq!(3, stack_set.n_of_stacks());
        assert_eq!(6, stack_set.size());
    }

    #[test]
    fn should_pop_sub_stack() {
        let mut stack_set = SetOfStacks::new(2);
        stack_set.push(1);
        stack_set.push(2);
        stack_set.push(3);
        stack_set.push(4);
        stack_set.push(5);
        stack_set.push(6);

        assert_eq!(3, stack_set.n_of_stacks());
        assert_eq!(2, stack_set.pop_at(0).unwrap());
        assert_eq!(4, stack_set.pop_at(1).unwrap());
        assert_eq!(6, stack_set.pop_at(2).unwrap());
        assert_eq!(3, stack_set.size());
    }
}
