//String Rotation:Assume you have a method isSubstring which checks if one word is a substring
//of another. Given two strings, sl and s2, write code to check if s2 is a rotation of sl using only one
//call to isSubstring (e.g., "waterbottle" is a rotation of"erbottlewat").

fn is_rotation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    is_substring(format!("{}{}", s1, s1).as_str(), s2)
}

fn is_substring(s1: &str, s2: &str) -> bool {
    s1.contains(s2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_a_rotation() {
        assert_eq!(is_rotation("waterbottle", "erbottlewat"), true);
    }

    #[test]
    fn should_not_be_a_rotation() {
        assert_eq!(is_rotation("potato", "tomato"), false);
    }
}
