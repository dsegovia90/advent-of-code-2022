use std::collections::HashSet;

const SIZE: usize = 14;

pub fn calculate(s: String) -> usize {
    for idx in 0..s.len() - SIZE {
        let set = &s[idx..idx + SIZE].chars().collect::<HashSet<_>>();
        if set.len() == SIZE {
            return idx + SIZE;
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
        assert_eq!(
            29,
            calculate("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string())
        );
        assert_eq!(
            26,
            calculate("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string())
        );
    }
}
