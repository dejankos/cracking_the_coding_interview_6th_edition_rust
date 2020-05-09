// Pairwise Swap: Write a program to swap odd and even bits in an integer with as few instructions as
// possible (e.g., bit 0 and bit 1 are swapped, bit 2 and bit 3 are swapped, and so on).

const ODD_MASK: u32 = 0xaa;
const EVEN_MASK: u32 = 0x55;

fn swap(n: u32) -> u32 {
    (n & ODD_MASK) >> 1 | (n & EVEN_MASK) << 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_swap_bits() {
        assert_eq!(swap(0b1001), 0b0110);
        assert_eq!(swap(0b0110), 0b1001);
    }
}
