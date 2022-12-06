use std::collections::HashSet;

const SIZE: usize = 4;


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
