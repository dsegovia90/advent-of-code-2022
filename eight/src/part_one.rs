#[derive(Debug)]
struct Tree {
    height: i32,
    visible: bool,
}

pub fn calculate(s: String) -> i32 {
    let lines = s.lines();
    let mut grid: Vec<_> = lines
        .map(|line| {
            line.chars()
                .map(|item| Tree {
                    height: item.to_digit(10).unwrap() as i32,
                    visible: false,
                })
                .collect::<Vec<_>>()
        })
        .collect();

    for row in grid.iter_mut() {
        let mut max = -1;
        for tree in row.iter_mut() {
            if tree.height > max {
                tree.visible = true;
            }
            max = std::cmp::max(max, tree.height);
        }

        let mut max = -1;
        for tree in row.iter_mut().rev() {
            if tree.height > max {
                tree.visible = true;
            }
            max = std::cmp::max(max, tree.height);
        }
    }

    for index in 0..grid[0].len() - 1 {
        let mut max = -1;
        for tree_row in grid.iter_mut() {
            let mut tree = &mut tree_row[index];
            if tree.height > max {
                tree.visible = true;
            }
            max = std::cmp::max(max, tree.height);
        }

        let mut max = -1;
        for tree_row in grid.iter_mut().rev() {
            let mut tree = &mut tree_row[index];
            if tree.height > max {
                tree.visible = true;
            }
            max = std::cmp::max(max, tree.height);
        }
    }

    let visible = grid.iter().fold(0, |acc, row| {
        acc + row
            .iter()
            .fold(0, |acc, tree| if tree.visible { acc + 1 } else { acc })
    });

    visible
}

#[cfg(test)]
mod test {
    use super::calculate;

    #[test]
    fn base_case() {
        let input = "30373\n25512\n65332\n33549\n35390".to_string();

        assert_eq!(calculate(input), 21);
    }
}
