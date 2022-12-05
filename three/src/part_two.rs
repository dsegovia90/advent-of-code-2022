use std::collections::HashSet;

const ALPHA: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn calculate(s: String) -> u32 {
    let lines: Vec<_> = s.lines().collect();

    let mut sum = 0;
    for chunk in lines.chunks(3) {
        sum += find_repeating(chunk);
    }

    sum
}

fn find_repeating(chunk: &[&str]) -> u32 {
    let mut letters: Vec<char> = chunk[0].chars().into_iter().collect();

    for chars in chunk {
        let unique: HashSet<char> = HashSet::from_iter(chars.chars());
        letters = unique
            .intersection(&letters.into_iter().collect())
            .copied()
            .collect()
    }

    let letter = letters.first().unwrap();
    (ALPHA.find(*letter).unwrap() + 1).try_into().unwrap()
}

#[cfg(test)]
mod test {
    use super::calculate;

    #[test]
    fn base_case() {
        let case = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw".to_string();

        let result = calculate(case);

        assert_eq!(result, 70)
    }
}
