// Towers of Hanoi: In the classic problem of the Towers of Hanoi, you have 3 towers and N disks of
// different sizes which can slide onto any tower. The puzzle starts with disks sorted in ascending order
// of size from top to bottom (i.e., each disk sits on top of an even larger one). You have the following
// constraints:
// (1) Only one disk can be moved at a time.
// (2) A disk is slid off the top of one tower onto another tower.
// (3) A disk cannot be placed on top of a smaller disk.
// Write a program to move the disks from the first tower to the last using Stacks.

fn move_disks(n: usize, from: &mut Vec<usize>, to: &mut Vec<usize>, buffer: &mut Vec<usize>) {
    if n > 0 {
        move_disks(n - 1, from, buffer, to);
        if let Some(disk) = from.pop() {
            to.push(disk);
        }
        move_disks(n - 1, buffer, to, from);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_puzzle() {
        let mut from = vec![3, 2, 1];
        let mut to = vec![];
        let mut buffer = vec![];

        move_disks(from.len(), &mut from, &mut to, &mut buffer);

        assert!(from.is_empty());
        assert!(buffer.is_empty());
        assert_eq!(vec![3, 2, 1], to);
    }
}
