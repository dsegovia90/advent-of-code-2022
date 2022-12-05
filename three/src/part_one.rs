use std::collections::HashSet;

const ALPHA: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn calculate(s: String) -> u32 {
    let lines = s.lines();

    let mut sum = 0;
    for line in lines {
        let line = line;
        let left = &line[..line.len() / 2];
        let right = &line[line.len() / 2..];
        sum += find_repeating(left, right);
    }
    sum
}

fn find_repeating(left: &str, right: &str) -> u32 {
    let left = left.chars();

    let left_letters: HashSet<_> = HashSet::from_iter(left);

    for char in right.chars() {
        if left_letters.contains(&char) {
            return (ALPHA.find(char).unwrap() + 1).try_into().unwrap();
        }
    }

    0
}

#[cfg(test)]
mod test {
    use super::calculate;

    #[test]
    fn base_case() {
        let case = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw".to_string();

        let result = calculate(case);

        assert_eq!(result, 157)
    }
}
