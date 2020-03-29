//Insertion: You are given two 32 - bit numbers, N and M, and two bit positions, i and
//j. Write a method to insert M into N such that M starts at bit j and ends at bit i. You
//can assume that the bits j through i have enough space to fit all of M. That is, if
//M = 10011, you can assume that there are at least 5 bits between j and i. You would not, for
//example, have j = 3 and i = 2, because M could not fully fit between bit 3 and bit 2.
//
//EXAMPLE
//Input: N = 10000000000, M = 10011, i = 2, j = 6
//Output: N = 10001001100

fn insert_bits(n: u32, m: u32, i: u32, j: u32) -> u32 {
    let left = !0 << (j + 1);
    let right = (1 << i) - 1;
    let mask = left | right; // bit mask to clear bits from j to i
    let cleared = n & mask;
    let m_shifted = m << i; // set to right position before merging

    cleared | m_shifted // merge bits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_insert() {
        assert_eq!(insert_bits(0b10001111100, 0b10011, 2, 6), 0b10001001100);
    }
}

fn main() {
    println!("insert bits - {:#b}", insert_bits(0b10001111100, 0b10011, 2, 6));
}