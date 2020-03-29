//String Compression: Implement a method to perform basic string compression using the counts
//of repeated characters. For example, the string aabcccccaaa would become a2b1c5a3. If the
//"compressed" string would not become smaller than the original string, your method should return
//the original string. You can assume the string has only uppercase and lowercase letters (a - z).


fn compress(s: &str) -> String {
    if s.len() == 1 {
        return String::from(s);
    }

    let c_vec: Vec<char> = s.chars().collect();
    let mut compressed = String::new();
    let mut iter = 1;
    let mut count = 1;

    while iter < c_vec.len() {
        if c_vec[iter] == c_vec[iter - 1] {
            count += 1;
        }
        if c_vec[iter] != c_vec[iter - 1] || iter == c_vec.len() - 1 {
            compressed.push(c_vec[iter - 1]);
            compressed.push_str(&count.to_string());
            count = 1;
        }

        iter += 1;
    }

    if compressed.len() > s.len() {
        return String::from(s);
    }

    compressed
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_compress() {
        assert_eq!(compress("aabcccccaaa"), "a2b1c5a3");
        assert_eq!(compress("zzzzzz"), "z6");
    }

    #[test]
    fn should_not_compress() {
        assert_eq!(compress("abcd"), "abcd");
    }
}
