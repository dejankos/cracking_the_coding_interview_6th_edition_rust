//One Away: There are three types of edits that can be performed on strings: insert a character,
//remove a character, or replace a character. Given two strings, write a function to check if they are
//one edit (or zero edits) away.
//EXAMPLE
//pale, ple  -> true
//pales,pale -> true
//pale, bale -> true
//pale, bake -> false

fn is_one_away(s1: &str, s2: &str) -> bool {
    if s1 == s2 {
        return true;
    };

    if ((s1.len() as i32) - (s2.len() as i32)).abs() > 1 {
        return false;
    };

    can_insert_update_delete(s1, s2)
}

fn can_insert_update_delete(s1: &str, s2: &str) -> bool {
    let (s, l) = get_ordered(s1, s2);

    let shortest: Vec<char> = s.chars().collect();
    let longer: Vec<char> = l.chars().collect();
    let mut iter = 0;

    while iter < shortest.len() {
        if shortest[iter] != longer[iter] {
            let s_offset = if s.len() == l.len() { 1 } else { 0 };
            let l_offset = 1;

            return shortest[iter + s_offset..] == longer[iter + l_offset..];
        }

        iter += 1;
    }

    true
}

fn get_ordered<'a>(s1: &'a str, s2: &'a str) -> (&'a str, &'a str) {
    if s1.len() == s2.len() {
        return (s1, s2);
    }

    return (get(|| { s1.len() < s2.len() }, s1, s2), get(|| { s1.len() > s2.len() }, s1, s2));
}

fn get<'a, F>(f: F, s1: &'a str, s2: &'a str) -> &'a str
    where F: Fn() -> bool
{
    if f() { s1 } else { s2 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_one_away() {
        assert_eq!(is_one_away("pale", "ple"), true);
        assert_eq!(is_one_away("ple", "pale"), true);
        assert_eq!(is_one_away("pale", "bale"), true);
        assert_eq!(is_one_away("pales", "pale"), true);
    }

    #[test]
    fn should_not_be_one_away() {
        assert_eq!(is_one_away("pale", "bake"), false);
        assert_eq!(is_one_away("Zero hour nine AM", "And I'm gonna be high as a kite by then"), false);
    }
}