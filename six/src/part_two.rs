use std::collections::HashSet;

const SIZE: usize = 14;

pub fn calculate(s: String) -> usize {
    let vec = s.chars().collect::<Vec<char>>();
    for (idx, item) in vec.windows(SIZE).enumerate() {
        if item.iter().collect::<HashSet<_>>().len() == SIZE {
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
