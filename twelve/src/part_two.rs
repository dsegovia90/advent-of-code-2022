use std::collections::{HashSet, VecDeque};

const ABC: &str = "abcdefghijklmnopqrstuvwxyz";

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

pub fn calculate_single(s: String) -> i32 {
    let lines = s.lines();
    let mut start = Coord { x: 0, y: 0 };
    let grid: Vec<Vec<i32>> = lines
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| {
                    if char == 'S' {
                        start = Coord { x, y };
                        return -1;
                    }
                    if char == 'E' {
                        return 26;
                    }
                    ABC.find(char).unwrap() as i32
                })
                .collect()
        })
        .collect();

    // grid.iter().for_each(|row| println!("{:?}", row));

    let mut visited = HashSet::from([start.clone()]);
    let mut queue = VecDeque::from([start]);
    let mut moves = 0;

    while !queue.is_empty() {
        let coords = queue.clone();
        queue.clear();

        for coord in coords.iter() {
            visited.insert(coord.clone());
            let val = grid[coord.y][coord.x];
            if val == 26 {
                return moves;
            }

            if coord.x > 0 && grid[coord.y][coord.x - 1] - 1 <= val {
                let new_coord = Coord {
                    x: coord.x - 1,
                    y: coord.y,
                };
                if !visited.contains(&new_coord) {
                    visited.insert(new_coord.clone());
                    queue.push_back(new_coord);
                }
            }

            if coord.x + 1 < grid[coord.y].len() && grid[coord.y][coord.x + 1] - 1 <= val {
                let new_coord = Coord {
                    x: coord.x + 1,
                    y: coord.y,
                };
                if !visited.contains(&new_coord) {
                    visited.insert(new_coord.clone());
                    queue.push_back(new_coord);
                }
            }

            if coord.y > 0 && grid[coord.y - 1][coord.x] - 1 <= val {
                let new_coord = Coord {
                    x: coord.x,
                    y: coord.y - 1,
                };
                if !visited.contains(&new_coord) {
                    visited.insert(new_coord.clone());
                    queue.push_back(new_coord);
                }
            }

            if coord.y + 1 < grid.len() && grid[coord.y + 1][coord.x] - 1 <= val {
                let new_coord = Coord {
                    x: coord.x,
                    y: coord.y + 1,
                };
                if !visited.contains(&new_coord) {
                    visited.insert(new_coord.clone());
                    queue.push_back(new_coord);
                }
            }
        }

        moves += 1;
    }

    99999999
}

pub fn calculate(s: String) -> i32 {
    let s = s.replace('S', "a");

    let indexes = s
        .chars()
        .enumerate()
        .filter_map(|(idx, char)| {
            if char == 'a' {
                return Some(idx);
            }
            None
        });


    indexes.map(|index| {
        let mut s = s.clone();
        s.replace_range(index..index + 1, "S");
        calculate_single(s.clone())
    }).min().unwrap() - 1
}

#[cfg(test)]
mod test {
    use super::calculate;

    #[test]
    fn base_case() {
        assert_eq!(
            calculate("Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi".to_string()),
            29
        );
    }
}
