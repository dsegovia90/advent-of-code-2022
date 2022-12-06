use std::collections::HashSet;

pub fn calculate(s: String) -> usize {

    for idx in 0..s.len() - 14 {
        let substring = &s[idx..idx + 14];
        let set: HashSet<_> = HashSet::from_iter(substring.chars());
        if set.len() == 14 {
            return idx + 14
        }
    }

    0
}

#[cfg(test)]
mod test {
    use super::calculate;

    #[test]
    fn base_case() {
        assert_eq!(19, calculate("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()));
        assert_eq!(23, calculate("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()));
        assert_eq!(23, calculate("nppdvjthqldpwncqszvftbrmjlhg".to_string()));
        assert_eq!(29, calculate("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()));
        assert_eq!(26, calculate("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()));
    }
}
