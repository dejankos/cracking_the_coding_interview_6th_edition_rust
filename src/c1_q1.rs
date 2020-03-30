//Is Unique: Implement an algorithm to determine if a string has all unique characters. What if you
//cannot use additional data structures?
use std::collections::HashSet;

fn has_unique_brute_force(s: &str) -> bool {
    let c_vec: Vec<char> = s.chars().collect();

    for (i, c) in c_vec.iter().enumerate() {
        for c_next in c_vec.iter().skip(i + 1) {
            if c == c_next {
                return false;
            }
        }
    }

    true
}

fn has_unique_store_in_hash_set(s: &str) -> bool {
    let c_vec = s.chars();
    let mut h_set = HashSet::new();

    for c in c_vec {
        if h_set.contains(&c) {
            return false;
        }

        h_set.insert(c);
    }

    true
}

fn has_unique_store_in_array_hash(s: &str) -> bool {
    let c_vec = s.chars();
    let mut arr: [u8; 128] = [u8::min_value(); 128];

    for c in c_vec {
        let c_int_value = c as usize;
        if arr[c_int_value] > 0 {
            return false;
        }

        arr[c_int_value] += 1;
    }

    true
}

fn has_unique_sort_and_check_neighbour(s: &str) -> bool {
    let mut c_vec: Vec<char> = s.chars().collect();
    let i_max = c_vec.len() - 1;
    c_vec.sort();

    for (i, c) in c_vec.iter().enumerate() {
        if i < i_max - 1 && c == &c_vec[i + 1] {
            return false;
        }
    }

    true
}

//todo
fn _has_unique_bit_vector(_s: &str) -> bool {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static UNIQUE_STR: &str = "Rust";
    static NON_UNIQUE_STR: &str = "Potato";

    #[test]
    fn should_be_unique() {
        assert_eq!(has_unique_brute_force(UNIQUE_STR), true);
        assert_eq!(has_unique_store_in_hash_set(UNIQUE_STR), true);
        assert_eq!(has_unique_store_in_array_hash(UNIQUE_STR), true);
        assert_eq!(has_unique_sort_and_check_neighbour(UNIQUE_STR), true);
    }

    #[test]
    fn should_not_be_unique() {
        assert_eq!(has_unique_brute_force(NON_UNIQUE_STR), false);
        assert_eq!(has_unique_store_in_hash_set(NON_UNIQUE_STR), false);
        assert_eq!(has_unique_store_in_array_hash(NON_UNIQUE_STR), false);
        assert_eq!(has_unique_sort_and_check_neighbour(NON_UNIQUE_STR), false);
    }
}