// Binary to String: Given a real number between O and 1 (e.g., 0.72) that is passed in as a double, print
// the binary representation. If the number cannot be represented accurately in binary with at most 32
// characters, print "ERROR:'

fn convert(mut d: f32) -> String {
    let mut s = String::from("0.");
    while d > 0.0 {
        if s.len() == 32 {
            panic!("Binary > 32 chars!")
        }

        let m = d * 2.0;
        if m >= 1.0 {
            s.push_str("1");
            d = m - 1.0;
        } else {
            s.push_str("0");
            d = m;
        }
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert() {
        assert_eq!("0.000110011001100110011001101", convert(0.1));
        assert_eq!("0.01011", convert(0.34375));
        assert_eq!("0.1011100001010001111011", convert(0.72));
    }
}
