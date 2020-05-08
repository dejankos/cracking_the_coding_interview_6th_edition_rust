// Conversion: Write a function to determine the number of bits you would need to flip to convert
// integer A to integer B.
// EXAMPLE
// Input: 29 (or: 11101), 15 (or: 01111)
// Output: 2

use crate::c5_q4::c_bits;

fn n_flip(a: u32, b: u32) -> u32 {
    c_bits(a ^ b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_number_of_bits_to_flip() {
        assert_eq!(n_flip(29, 15), 2);
        assert_eq!(n_flip(0, 63), 6);
    }
}
