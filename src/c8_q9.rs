// Parens: Implement an algorithm to print all valid (e.g., properly opened and closed) combinations
// of n pairs of parentheses.

use std::collections::HashSet;

fn gen_parens(n: usize) -> HashSet<String> {
    n_pairs(n).into_iter().collect::<HashSet<String>>()
}

fn n_pairs(n: usize) -> Vec<String> {
    if n == 1 {
        vec!["()".to_string()]
    } else {
        n_pairs(n - 1)
            .iter()
            .map(|sub| rotate(sub.clone()))
            .flatten()
            .collect::<Vec<String>>()
    }
}

pub fn rotate(s: String) -> Vec<String> {
    let mut res = s
        .char_indices()
        .map(|(i, c)| {
            if c == '(' {
                let p1 = &s[0..i + 1];
                let p2 = &s[i + 1..s.len()];
                format!("{}(){}", p1, p2)
            } else {
                "".to_string()
            }
        })
        .filter(|r| !r.is_empty())
        .collect::<Vec<String>>();
    res.push(format!("(){}", s));

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn should_gen_n_pairs() {
        assert_eq!(
            HashSet::from_iter(vec![
                "(()())".to_string(),
                "()(())".to_string(),
                "()()()".to_string(),
                "(())()".to_string(),
                "((()))".to_string()
            ]),
            gen_parens(3)
        );
    }
}
