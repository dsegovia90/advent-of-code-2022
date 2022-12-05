#[derive(Debug)]
struct Range {
    a: u32,
    b: u32,
}

impl Range {
    fn new(vec: Vec<u32>) -> Self {
        Self {
            a: vec[0],
            b: vec[1],
        }
    }

    fn intersect(&self, other: &Self) -> bool {
        self.a >= other.a && self.b <= other.b || other.a >= self.a && other.b <= self.b
    }
}

pub fn calculate(s: String) -> u32 {
    let lines = s.lines();

    let mut sum = 0;
    for line in lines {
        let line: Vec<_> = line.split(',').collect();
        let left = Range::new(
            line[0]
                .split('-')
                .map(|item| item.parse::<u32>().unwrap())
                .collect(),
        );
        let right = Range::new(
            line[1]
                .split('-')
                .map(|item| item.parse::<u32>().unwrap())
                .collect(),
        );

        if left.intersect(&right) {
            sum += 1
        };
    }
    sum
}

#[cfg(test)]
mod test {
    use super::calculate;

    #[test]
    fn base_case() {
        let case = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8".to_string();

        let result = calculate(case);

        assert_eq!(result, 2)
    }
}
