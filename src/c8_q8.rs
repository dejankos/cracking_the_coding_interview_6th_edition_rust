// Permutations with Duplicates: Write a method to compute all permutations of a string whose
// characters are not necessarily unique. The list of permutations should not have duplicates

use crate::c8_q7;

fn str_per(mut s: String, lookup: &mut Vec<u8>) -> Vec<String> {
    if s.is_empty() {
        vec![String::from("")]
    } else {
        let c = s.pop().unwrap();
        let subs = str_per(s, lookup);
        if lookup[c as usize] == 1 {
            subs
        } else {
            lookup[c as usize] = 1;
            subs.iter()
                .map(|sub| {
                    if sub.is_empty() {
                        vec![c.to_string()]
                    } else {
                        c8_q7::rotate(sub.clone(), c)
                    }
                })
                .flatten()
                .collect::<Vec<String>>()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_generate_permutations_without_dups() {
        let r = str_per(String::from("abaabcc"), &mut vec![0; 127]);
        assert_eq!(vec!["abc", "cab", "acb", "bac", "cba", "bca"], r);
    }
}
