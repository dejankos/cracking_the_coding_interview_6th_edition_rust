// Permutations without Dups: Write a method to compute all permutations of a string of unique
// characters

fn str_per(mut s: String) -> Vec<String> {
    if s.is_empty() {
        vec![String::from("")]
    } else {
        let c = s.pop().unwrap();
        let subs = str_per(s);

        subs.iter()
            .map(|sub| {
                if sub.is_empty() {
                    vec![c.to_string()]
                } else {
                    rotate(sub.clone(), c)
                }
            })
            .flatten()
            .collect::<Vec<String>>()
    }
}

fn rotate(s: String, c: char) -> Vec<String> {
    let mut res = vec![];
    res.push(format!("{}{}", s, c));

    let mut ast = s
        .char_indices()
        .map(|(i, _)| {
            let p1 = &s[0..i];
            let p2 = &s[i..s.len()];
            format!("{}{}{}", p1, c, p2)
        })
        .collect::<Vec<String>>();

    res.append(&mut ast);

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_generate_permutations() {
        let r = str_per(String::from("abc"));
        assert_eq!(vec!["abc", "cab", "acb", "bac", "cba", "bca"], r);
    }
}
