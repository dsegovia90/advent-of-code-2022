use std::{collections::HashSet, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Copy)]
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
    let mut tails = [Point::default(); 10];

    let moves = s.lines().map(|line| {
        let mut iter = line.split(' ');
        let dir = Direction::from_str(iter.next().unwrap()).unwrap();
        let spaces = iter.next().unwrap().parse::<i32>().unwrap();
        (dir, spaces)
    });

    moves.for_each(|(dir, spaces)| {
        for _ in 0..spaces {
            match dir {
                Direction::Left => {
                    tails[0].x -= 1;
                }
                Direction::Up => {
                    tails[0].y += 1;
                }
                Direction::Right => {
                    tails[0].x += 1;
                }
                Direction::Down => {
                    tails[0].y -= 1;
                }
            }

            for i in 0..9 {
                let head = tails[i];
                let mut tail = &mut tails[i + 1];

                let distance_x = (head.x - tail.x).abs();
                let distance_y = (head.y - tail.y).abs();

                if distance_x > 1 && head.y == tail.y {
                    if head.x < tail.x {
                        tail.x -= 1;
                    } else {
                        tail.x += 1;
                    }
                } else if distance_y > 1 && head.x == tail.x {
                    if head.y < tail.y {
                        tail.y -= 1;
                    } else {
                        tail.y += 1;
                    }
                } else if head.x - tail.x == 2 && head.y - tail.y == 2
                    || head.x - tail.x == 1 && head.y - tail.y == 2
                    || head.x - tail.x == 2 && head.y - tail.y == 1
                {
                    tail.x += 1;
                    tail.y += 1;
                } else if head.x - tail.x == -2 && head.y - tail.y == 2
                    || head.x - tail.x == -1 && head.y - tail.y == 2
                    || head.x - tail.x == -2 && head.y - tail.y == 1
                {
                    tail.x -= 1;
                    tail.y += 1;
                } else if head.x - tail.x == 2 && head.y - tail.y == -2
                    || head.x - tail.x == 1 && head.y - tail.y == -2
                    || head.x - tail.x == 2 && head.y - tail.y == -1
                {
                    tail.x += 1;
                    tail.y -= 1;
                } else if head.x - tail.x == -2 && head.y - tail.y == -2
                    || head.x - tail.x == -1 && head.y - tail.y == -2
                    || head.x - tail.x == -2 && head.y - tail.y == -1
                {
                    tail.x -= 1;
                    tail.y -= 1;
                }
            }

            tail_unique_positions.insert(tails[9]);
        }
    });

    tail_unique_positions.len()
}

fn printer(arr: [Point; 10]) {
    let mut screen = [['.'; 6]; 5];

    arr.iter().enumerate().rev().for_each(|(idx, point)| {
        screen[4 - point.y as usize][point.x as usize] = char::from_digit(idx as u32, 10).unwrap()
    });

    screen.iter().for_each(|row| {
        row.iter().for_each(|char| print!("{char}"));
        println!();
    });
}

#[cfg(test)]
mod test {
    use super::calculate;

    #[test]
    fn base_case() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2".to_string();

        assert_eq!(calculate(input), 1);

        let input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20".to_string();
        assert_eq!(calculate(input), 36);
    }
}
