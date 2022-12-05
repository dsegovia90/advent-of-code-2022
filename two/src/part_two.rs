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
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Move {
    X = 0,
    Y = 3,
    Z = 6,
}



impl FromStr for Move {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "X" => Ok(Self::X),
            "Y" => Ok(Self::Y),
            "Z" => Ok(Self::Z),
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

fn our_move(opponent: &Options, we_must: &Move) -> Options {
    match we_must {
        Move::X => match opponent {
            Options::Rock => Options::Scissors,
            Options::Paper => Options::Rock,
            Options::Scissors => Options::Paper,
        },
        Move::Y => *opponent,
        Move::Z => match opponent {
            Options::Rock => Options::Paper,
            Options::Paper => Options::Scissors,
            Options::Scissors => Options::Rock,
        },
    }
}

pub fn calculate_points(input: String) -> u32 {
    let lines: Vec<_> = input
        .lines()
        .map(|line| {
            let split: Vec<_> = line.split(' ').collect();
            let opponent = Options::from_str(split[0]).unwrap();
            let we_must = Move::from_str(split[1]).unwrap();
            let our_move = our_move(&opponent, &we_must);
            choose_points(&our_move) + match_points(&opponent, &our_move)
        })
        .collect();

    lines.iter().sum()
}

#[cfg(test)]
mod test {
    use crate::part_two::calculate_points;

    #[test]
    fn base_case() {
        let case = "A Y\nB X\nC Z".to_string();
        let result = calculate_points(case);

        assert_eq!(result, 12);
    }
}
