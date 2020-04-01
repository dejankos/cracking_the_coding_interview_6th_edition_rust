// Sum Lists: You have two numbers represented by a linked list, where each node contains a single
// digit. The digits are stored in reverse order, such that the 1 's digit is at the head of the list. Write a
// function that adds the two numbers and returns the sum as a linked list.
// Sum Lists: You have two numbers represented by
// EXAMPLE
// Input: (7-> 1 -> 6) + (5 -> 9 -> 2).That is,617 + 295.
// Output: 2 -> 1 -> 9. That is, 912.
// FOLLOW UP
// Suppose the digits are stored in forward order. Repeat the above problem.
// EXAMPLE
// Input:(6 -> 1 -> 7) + (2 -> 9 -> 5).That is,617 + 295.
// Output: 9 -> 1 -> 2. That is, 912.


use crate::linked_list::{LinkedList, List};

fn to_number(list: &mut LinkedList<u32>) -> u32 {
    list.into_iter()
        .map(|rc| rc.borrow_mut().e.clone().to_string())
        .collect::<String>()
        .chars()
        //can be simplified by implementing LL DoubleEndedIterator
        .rev()
        .collect::<String>()
        .parse::<u32>()
        .unwrap()
}

fn from_number(n: u32) -> LinkedList<u32> {
    n.to_string()
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn sum(list1: &mut LinkedList<u32>, list2: &mut LinkedList<u32>) -> LinkedList<u32> {
    from_number(to_number(list1) + to_number(list2))
}

// FOLLOW UP
// Suppose the digits are stored in forward order. Repeat the above problem.
//
// Same just without the rev iterator

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_to_number_reversed() {
        let mut list: LinkedList<u32> = LinkedList::new();
        list.add(7);
        list.add(1);
        list.add(6);

        assert_eq!(to_number(&mut list), 617);
    }

    #[test]
    fn should_convert_from_number_reversed() {
        assert_eq!(format!("{}", from_number(219)), "[9, 1, 2]")
    }

    #[test]
    fn should_sum_lists_reversed() {
        let (mut list1, mut list2): (LinkedList<u32>, LinkedList<u32>) = (LinkedList::new(), LinkedList::new());
        list1.add(7);
        list1.add(1);
        list1.add(6);

        list2.add(5);
        list2.add(9);
        list2.add(2);

        let mut sum = sum(&mut list1, &mut list2);
        assert_eq!(format!("{}", sum), "[2, 1, 9]");
        assert_eq!(to_number(&mut sum), 912);
    }
}