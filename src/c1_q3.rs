//URLify: Write a method to replace all spaces in a string with '%20'. You may assume that the string
//has sufficient space at the end to hold the additional characters, and that you are given the "true"
//length of the string. (Note: If implementing in Java, please use a character array so that you can
//perform this operation in place.)
//EXAMPLE
//
//Input: "Mr John Smith     ", 13
//Output: "Mr%20John%20Smith"

const DELIMITER: &str = "%20";
const WHITE_SPACE: char = ' ';

fn replace(s: &str) -> String {
    s.trim_end().replace(WHITE_SPACE, DELIMITER)
}

fn replace_2(s: &str) -> String {
    let mut res = String::new();
    let split: Vec<&str> = s.split_whitespace().collect();

    for (i, s) in split.iter().enumerate() {
        res.push_str(s);


        if i < split.len() - 1 {
            res.push_str(DELIMITER);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_replaced() {
        assert_eq!(replace("Mr John Smith     "), "Mr%20John%20Smith");
        assert_eq!(replace_2("Mr John Smith     "), "Mr%20John%20Smith");
    }
}
