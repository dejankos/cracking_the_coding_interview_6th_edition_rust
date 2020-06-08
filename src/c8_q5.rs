// Recursive Multiply: Write a recursive function to multiply two positive integers without using
// the * operator (or / operator). You can use addition, subtraction, and bit shifting, but you should
// minimize the number of those operations.

fn multiply(a: isize, b: isize) -> isize {
    let r = r_multiply(a.abs() as usize, b.abs() as usize);

    (if (a < 0) ^ (b < 0) { !(r - 1) } else { r }) as isize
}

fn r_multiply(a: usize, b: usize) -> usize {
    if b == 1 {
        a
    } else {
        a + r_multiply(a, b - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_multiply() {
        assert_eq!(4, multiply(2, 2));
        assert_eq!(6, multiply(2, 3));
        assert_eq!(12, multiply(4, 3));
        assert_eq!(30, multiply(10, 3));
        assert_eq!(4, multiply(-2, -2));
        assert_eq!(-4, multiply(2, -2));
        assert_eq!(16, multiply(-4, -4));
        assert_eq!(-16, multiply(-4, 4));
    }
}
