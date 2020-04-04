// Write a program to sort a stack such that the smallest items are on the top. You can use
// an additional temporary stack, but you may not copy the elements into any other data structure
// (such as an array). The stack supports the following operations: push, pop, peek, and is Empty.

use crate::vec_stack::{Stack, VecStack};

fn sort_stack(stack: &mut VecStack<usize>) {
    let mut t_stack: VecStack<usize> = VecStack::new();
    let mut min = usize::max_value();
    let mut l_min = min;

    // find the min element in every iteration store to temp in reverse sorted order
    // and copy to original stack giving right order
    loop {
        while let Some(v) = stack.pop() {
            if v < min {
                min = v;
            }

            t_stack.push(v);
        }

        while let Some(v) = t_stack.peek() {
            if v == l_min {
                break;
            }

            let head = t_stack.pop().unwrap();
            if head != min {
                stack.push(head);
            }
        }
        t_stack.push(min);
        l_min = min;
        min = usize::max_value();

        if stack.peek().is_none() {
            break;
        }
    }

    while let Some(v) = t_stack.pop() {
        stack.push(v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_sort_stack() {
        let mut stack = VecStack::new();
        stack.push(1);
        stack.push(5);
        stack.push(2);
        stack.push(4);
        stack.push(3);

        sort_stack(&mut stack);

        assert_eq!(1, stack.pop().unwrap());
        assert_eq!(2, stack.pop().unwrap());
        assert_eq!(3, stack.pop().unwrap());
        assert_eq!(4, stack.pop().unwrap());
        assert_eq!(5, stack.pop().unwrap());
    }
}
