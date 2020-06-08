// Power Set: Write a method to return all subsets of a set.

fn gen_subset(set: Vec<usize>) -> Vec<Vec<usize>> {
    r_gen_subset(&set, 0)
}

fn r_gen_subset(set: &Vec<usize>, i: usize) -> Vec<Vec<usize>> {
    if i == set.len() - 1 {
        vec![vec![set[i]]]
    } else {
        let prev = r_gen_subset(set, i + 1);
        let mut new = prev
            .clone()
            .iter_mut()
            .map(|v| {
                v.push(set[i]);
                v.clone()
            })
            .collect::<Vec<Vec<usize>>>();
        new.push(vec![set[i]]);

        [&prev[..], &new[..]].concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_gen_all_subsets() {
        let set = vec![1, 2, 3];
        assert_eq!(
            vec![[3], [3, 2], [2], [3, 1], [3, 2, 1], [2, 1], [1]],
            gen_subset(set)
        );
    }
}
