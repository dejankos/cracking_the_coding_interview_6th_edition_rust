//Palindrome Permutation: Given a string, write a function to check if it is a permutation of a palin­
//drome. A palindrome is a word or phrase that is the same forwards and backwards. A permutation
//is a rearrangement of letters. The palindrome does not need to be limited to just dictionary words.
//EXAMPLE
//Input: Tact Coa
//Output: True (permutations: "taco cat", "atco eta", etc.)

use std::collections::HashMap;

const WHITE_SPACE: char = ' ';

fn is_palindrome_permutation(s: &str) -> bool {
    let mut map: HashMap<char, u8> = HashMap::new();

    for c in s.chars() {
        if c == WHITE_SPACE {
            continue;
        }

        let c = c.to_ascii_lowercase();
        *map.entry(c).or_insert(0) += 1;
    }

    let mut odd_count = 0;
    for v in map.values() {
        if v % 2 > 0 {
            odd_count += 1;
        }
    }

    odd_count <= 1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_a_palindrome_permutation() {
        assert_eq!(is_palindrome_permutation("Tact Coa"), true);
    }

    #[test]
    fn should_not_be_a_palindrome_permutation() {
        assert_eq!(is_palindrome_permutation("She packed my bags last night pre-flight..."), false);
    }
}
