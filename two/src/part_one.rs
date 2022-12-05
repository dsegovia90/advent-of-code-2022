use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Options {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Options {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

fn choose_points(us: &Options) -> u32 {
    *us as u32
}

fn match_points(opponent: &Options, us: &Options) -> u32 {
    match opponent {
        Options::Rock => match us {
            Options::Rock => 3,
            Options::Paper => 6,
            Options::Scissors => 0,
        },
        Options::Paper => match us {
            Options::Rock => 0,
            Options::Paper => 3,
            Options::Scissors => 6,
        },
        Options::Scissors => match us {
            Options::Rock => 6,
            Options::Paper => 0,
            Options::Scissors => 3,
        },
    }
}

pub fn calculate_points(input: String) -> u32 {
    let lines: Vec<_> = input
        .lines()
        .map(|line| {
            let split: Vec<_> = line.split(' ').collect();
            let opponent = Options::from_str(split[0]).unwrap();
            let us = Options::from_str(split[1]).unwrap();
            choose_points(&us) + match_points(&opponent, &us)
        })
        .collect();

    lines.iter().sum()
}

#[cfg(test)]
mod test {
    use crate::part_one::calculate_points;

    #[test]
    fn base_case() {
        let case = "A Y\nB X\nC Z".to_string();
        let result = calculate_points(case);

        assert_eq!(result, 15);
    }
}
