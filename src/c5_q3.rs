// Flip Bit to Win: You have an integer and you can flip exactly one bit from a O to a 1. Write code to
// find the length of the longest sequence of 1 s you could create.
// EXAMPLE
// Input: 1775
// Output: 8

fn find_max_sequence(n: u32) -> usize {
    let (mut max, mut current, mut flip) = (0, 0, false);
    for i in (0..32).rev() {
        let x = (n >> i) & 1;
        if x == 1 {
            current = current + 1;
        } else {
            if flip {
                flip = false;
                current = 0;
            } else {
                flip = true;
                current = current + 1;
            }
        }
        if current > max {
            max = current;
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_longest_sequence() {
        assert_eq!(8, find_max_sequence(0b11011101111));
        assert_eq!(4, find_max_sequence(0b1110));
        assert_eq!(3, find_max_sequence(0b101010));
        assert_eq!(9, find_max_sequence(0b10111111101));
    }
}
