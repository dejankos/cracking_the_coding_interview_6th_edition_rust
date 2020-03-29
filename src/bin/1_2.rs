//Check Permutation: Given two strings, write a method to decide if one is a permutation of the
//other.

fn is_per(s1: &str, s2: &str) -> bool {
    if diff_len(s1, s2) {
        return false;
    }

    vec_sort(s1) == vec_sort(s2)
}

fn is_per_hash(s1: &str, s2: &str) -> bool {
    if diff_len(s1, s2) {
        return false;
    }

    let mut arr: [i8; 128] = [0; 128];
    for c in s1.chars() {
        arr[c as usize] += 1;
    }

    for c in s2.chars() {
        arr[c as usize] -= 1;
        if arr[c as usize] < 0 {
            return false;
        }
    }

    true
}

fn vec_sort(s: &str) -> String {
    let mut c_vec: Vec<char> = s.chars().collect();
    c_vec.sort();
    c_vec.into_iter().collect::<String>()
}

fn diff_len(s1: &str, s2: &str) -> bool {
    s1.len() != s2.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_a_permutation() {
        assert_eq!(is_per("abc", "bca"), true);
        assert_eq!(is_per_hash("abc", "bca"), true);
    }

    #[test]
    fn should_not_be_a_permutation() {
        assert_eq!(is_per("abc", "aab"), false);
        assert_eq!(is_per_hash("abc", "aab"), false);
        assert_eq!(is_per("abc", "aabe"), false);
        assert_eq!(is_per_hash("abc", "aabe"), false);
    }
}

fn main() {
    println!(" is_per - {},\
     is_per_hash - {}",
             is_per("abc", "bca"),
             is_per_hash("abc", "bca")
    )
}