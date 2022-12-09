use std::{collections::HashSet, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::Left),
            "U" => Ok(Self::Up),
            "R" => Ok(Self::Right),
            "D" => Ok(Self::Down),
            _ => Err(()),
        }
    }
}

pub fn calculate(s: String) -> usize {
    let mut tail_unique_positions = HashSet::new();
    let mut head = Point {
        x: 0,
        y: 0,
    };
    let mut tail = Point {
        x: 0,
        y: 0,
    };

    let moves = s
        .lines()
        .map(|line| {
            let mut iter = line.split(' ');
            let dir = Direction::from_str(iter.next().unwrap()).unwrap();
            let spaces = iter.next().unwrap().parse::<i32>().unwrap();
            (dir, spaces)
        });

    moves.for_each(|(dir, spaces)| {
        for _ in 0..spaces {
            match dir {
                Direction::Left => {
                    head.x -= 1;
                },
                Direction::Up => {
                    head.y += 1;
                },
                Direction::Right => {
                    head.x += 1;
                },
                Direction::Down => {
                    head.y -= 1;
                },
            }


            let distance_x = (head.x - tail.x).abs();
            let distance_y = (head.y - tail.y).abs();

            if distance_x > 1 {
                match dir {
                    Direction::Left => tail.x -= 1,
                    Direction::Right => tail.x += 1,
                    _ => ()
                }
                tail.y = head.y;
            } else if distance_y > 1 {
                match dir {
                    Direction::Up => tail.y += 1,
                    Direction::Down => tail.y -= 1,
                    _ => ()
                }
                tail.x = head.x;
            }
            tail_unique_positions.insert(tail);
        }
    });

    tail_unique_positions.len()
}

#[cfg(test)]
mod test {
    use super::calculate;

    #[test]
    fn base_case() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2".to_string();

        assert_eq!(calculate(input), 13);
    }
}
