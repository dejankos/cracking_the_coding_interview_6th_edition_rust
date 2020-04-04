// Palindrome: Implement a function to check if a linked list is a palindrome.

use crate::linked_list::{LinkedList, List};

trait Palindrome<T>: List<T>
where
    T: PartialEq,
{
    fn is_palindrome(&mut self) -> bool;
}

impl<T> Palindrome<T> for LinkedList<T>
where
    T: PartialEq,
{
    fn is_palindrome(&mut self) -> bool {
        self.into_iter()
            .take((self.size() / 2) as usize)
            .zip(self.into_rev_iter())
            .all(|(l, r)| l.borrow().e == r.borrow().e)
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::{LinkedList, List};

    use super::*;

    #[test]
    fn should_be_a_palindrome() {
        let mut list = LinkedList::new();
        list.add(1);
        list.add(2);
        list.add(1);

        assert!(list.is_palindrome())
    }

    #[test]
    fn should_be_a_palindrome_chars() {
        let mut list = LinkedList::new();
        list.add('a');
        list.add('b');
        list.add('b');
        list.add('a');

        assert!(list.is_palindrome())
    }

    #[test]
    fn should_not_be_a_palindrome() {
        let mut list = LinkedList::new();
        list.add(1);
        list.add(2);
        list.add(3);

        assert!(!list.is_palindrome())
    }
}
