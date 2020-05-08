// Next Number: Given a positive integer, print the next smallest and the next largest number that
// have the same number of 1 bits in their binary representation

fn find_next(n: u32) -> (u32, u32) {
    let bits = c_bits(n);
    let (mut l, mut s) = (n.clone(), n.clone());
    (
        f_next(bits, || {
            l = l + 1;
            l
        }),
        f_next(bits, || {
            s = s - 1;
            s
        }),
    )
}

fn f_next<F>(bits: u32, mut f: F) -> u32
where
    F: FnMut() -> u32,
{
    let mut x = f();
    while c_bits(x) != bits {
        x = f();
    }
    x
}

pub fn c_bits(mut n: u32) -> u32 {
    let mut c = 0;
    while n > 0 {
        c += n & 1;
        n = n >> 1;
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_next_lower_and_grater() {
        let res = find_next(42);
        assert_eq!(44, res.0);
        assert_eq!(41, res.1);
    }
}
