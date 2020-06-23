// Coins: Given an infinite number of quarters (25 cents), dimes (10 cents), nickels (5 cents), and
// pennies (1 cent), write code to calculate the number of ways of representing n cents.

type VecMap = Vec<Vec<usize>>;

fn make_change(amount: usize, coins: &[usize]) -> usize {
    r_make_change(amount, coins, 0, &mut vec![vec![0; amount]; 127])
}

fn r_make_change(amount: usize, coins: &[usize], idx: usize, lookup: &mut VecMap) -> usize {
    if lookup[amount][idx] != 0 {
        lookup[amount][idx]
    } else if idx > coins.len() - 1 {
        1
    } else {
        let coin = coins[idx];
        let (mut ways, mut i) = (0, 0);
        while i * coin < amount {
            let remain = amount - (i * coin);
            ways += r_make_change(remain, coins, idx + 1, lookup);
            i += 1;
        }

        lookup[amount][idx] = ways;
        ways
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_make_change() {
        assert_eq!(make_change(4, &[1, 2, 3]), 7);
        assert_eq!(make_change(98, &[25, 10, 5, 1]), 6294);
    }
}
