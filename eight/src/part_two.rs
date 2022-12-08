#[derive(Debug)]
struct Tree {
    height: i32,
}

pub fn calculate(s: String) -> u32 {
    let lines = s.lines();
    let grid: Vec<_> = lines
        .map(|line| {
            line.chars()
                .map(|item| Tree {
                    height: item.to_digit(10).unwrap() as i32,
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut max = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, _tree) in row.iter().enumerate() {
            max = std::cmp::max(
                run_right(x, y, &grid)
                    * run_left(x, y, &grid)
                    * run_up(x, y, &grid)
                    * run_down(x, y, &grid),
                max,
            );
        }
    }

    max
}

fn run_right(initial_x: usize, initial_y: usize, grid: &Vec<Vec<Tree>>) -> u32 {
    if initial_y == 0 || initial_y >= grid.len() - 1 {
        return 0;
    }

    if initial_x == 0 || initial_x >= grid[initial_y].len() - 1 {
        return 0;
    }

    let height = grid[initial_y][initial_x].height;
    let mut sum = 0;
    for tree in grid[initial_y].iter().skip(initial_x + 1) {
        sum += 1;
        if tree.height >= height {
            return sum;
        }
    }

    sum
}

fn run_left(initial_x: usize, initial_y: usize, grid: &Vec<Vec<Tree>>) -> u32 {
    if initial_y == 0 || initial_y >= grid.len() - 1 {
        return 0;
    }

    if initial_x == 0 || initial_x >= grid[initial_y].len() - 1 {
        return 0;
    }

    let height = grid[initial_y][initial_x].height;
    let mut sum = 0;

    for tree in grid[initial_y].iter().take(initial_x).rev() {
        sum += 1;
        if tree.height >= height {
            return sum;
        }
    }

    sum
}

fn run_up(initial_x: usize, initial_y: usize, grid: &Vec<Vec<Tree>>) -> u32 {
    if initial_y == 0 || initial_y >= grid.len() - 1 {
        return 0;
    }

    if initial_x == 0 || initial_x >= grid[initial_y].len() - 1 {
        return 0;
    }

    let height = grid[initial_y][initial_x].height;
    let mut sum = 0;
    for row in grid.iter().take(initial_y).rev() {
        let new_height = row[initial_x].height;
        sum += 1;
        if new_height >= height {
            return sum;
        }
    }

    sum
}

fn run_down(initial_x: usize, initial_y: usize, grid: &Vec<Vec<Tree>>) -> u32 {
    if initial_y == 0 || initial_y >= grid.len() - 1 {
        return 0;
    }

    if initial_x == 0 || initial_x >= grid[initial_y].len() - 1 {
        return 0;
    }

    let height = grid[initial_y][initial_x].height;
    let mut sum = 0;
    for row in grid.iter().skip(initial_y + 1) {
        let new_height = row[initial_x].height;
        sum += 1;
        if new_height >= height {
            return sum;
        }
    }

    sum
}

#[cfg(test)]
mod test {
    use super::calculate;

    #[test]
    fn base_case() {
        let input = "30373\n25512\n65332\n33549\n35390".to_string();

        assert_eq!(calculate(input), 8);
    }
}
