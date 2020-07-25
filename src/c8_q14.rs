// Boolean Evaluation: Given a boolean expression consisting of the symbols 0 (false), 1 (true), &
// (AND), I (OR), and /\ (XOR), and a desired boolean result value result, implement a function to
// count the number of ways of parenthesizing the expression such that it evaluates to result. The
// expression should be fully parenthesized (e.g., ( 0) A( 1)) but not extraneously (e.g., ( ( ( 0)) /\ ( 1)) ).
// EXAMPLE
// countEval("l /\ 01011", false) -> 2
// countEval("0&0&0&1 A ll0", true)-> 10


fn count_eval(expr: &str, res: bool) -> usize {
    if expr.len() == 0 {
        return 0;
    }
    if expr.len() == 1 {
        return ((expr.chars().nth(0).unwrap_or('\0') == '1') == res) as usize;
    }

    let mut ways = 0;
    for (i, c) in expr.chars().enumerate().skip(1).step_by(2) {
        let left = &expr[0..i];
        let right = &expr[i + 1..expr.len()];

        let left_true = count_eval(left, true);
        let left_false = count_eval(left, false);
        let right_true = count_eval(right, true);
        let right_false = count_eval(right, false);
        let total = (left_true + left_false) * (right_true + right_false);

        let mut total_true = 0;
        match c {
            '^' => {
                total_true = left_true * right_false + left_false * right_true
            }
            '&' => {
                total_true = left_true * right_true
            }
            '|' => {
                total_true = left_true * right_true + left_false * right_true + left_true * right_false
            }
            _ => {
                panic!("unknown op ")
            }
        }

        let sub_ways = if res { total_true } else { total - total_true };
        ways += sub_ways;
    }

    ways
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_count_eval() {
        assert_eq!(10, count_eval("0^0&0^1|1", true));
        assert_eq!(2, count_eval("1^0|0|1", false));
    }
}