// Triple Step: A child is running up a staircase with n steps and can hop either 1 step, 2 steps, or 3
// steps at a time. Implement a method to count how many possible ways the child can run up the
// stairs.
fn count_ways(n: isize) -> isize {
    let mut m = vec![-1; (n + 1) as usize];
    r_count_ways(n, &mut m)
}

fn r_count_ways(n: isize, m: &mut Vec<isize>) -> isize {
    let i = n as usize;
    if n < 0 {
        0
    } else if n == 0 {
        1
    } else if m[i] != -1 {
        m[i]
    } else {
        m[i] = r_count_ways(n - 1, m) + r_count_ways(n - 2, m) + r_count_ways(n - 3, m);
        m[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_count_steps() {
        assert_eq!(2, count_ways(2));
        assert_eq!(4, count_ways(3));
        assert_eq!(7, count_ways(4));
        assert_eq!(13, count_ways(5));
    }
}
