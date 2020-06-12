// Magic Index: A magic index in an array A[ 1 .•. n-1] is defined to be an index such that A[ i]
// i. Given a sorted array of distinct integers, write a method to find a magic index, if one exists, in
// array A.

use std::cmp::Ordering;

const NOT_FOUND: usize = 99;

fn find_magic_index(arr: [isize; 10]) -> usize {
    r_magic_index(arr, 0, arr.len() - 1)
}

fn r_magic_index(arr: [isize; 10], s: usize, e: usize) -> usize {
    if s > e {
        return NOT_FOUND;
    }

    let mid = (s + e) / 2;
    match arr[mid].cmp(&(mid as isize)) {
        Ordering::Equal => mid,
        Ordering::Less => r_magic_index(arr, mid + 1, e),
        Ordering::Greater => {
            if mid == 0 {
                NOT_FOUND
            } else {
                r_magic_index(arr, s, mid - 1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_magic_index() {
        let arr = [-1, 0, 1, 2, 3, 4, 6, 8, 9, 10];
        assert_eq!(6, find_magic_index(arr));

        let arr = [-1, 0, 2, 4, 6, 7, 8, 9, 10, 11];
        assert_eq!(2, find_magic_index(arr));
    }

    #[test]
    fn should_not_find_magic_index() {
        let arr = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        assert_eq!(NOT_FOUND, find_magic_index(arr));
    }
}
