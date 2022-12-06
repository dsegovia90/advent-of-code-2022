use std::collections::HashSet;

pub fn calculate(s: String) -> usize {
    for idx in 0..s.len() - 4 {
        let substring = &s[idx..idx + 4];
        let set: HashSet<_> = HashSet::from_iter(substring.chars());
        if set.len() == 4 {
            return idx + 4;
        }
    }

    0
}

#[cfg(test)]
mod test {
    use super::calculate;

    #[test]
    fn base_case() {
        assert_eq!(7, calculate("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()));
        assert_eq!(5, calculate("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()));
        assert_eq!(6, calculate("nppdvjthqldpwncqszvftbrmjlhg".to_string()));
        assert_eq!(
            10,
            calculate("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string())
        );
        assert_eq!(
            11,
            calculate("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string())
        );
    }
}
