use std::vec;

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

#[derive(Debug)]
struct Dir {
    name: String,
    children: Vec<Children>,
    size: Option<u32>,
}

impl Dir {
    fn append_dir(&mut self, name: String) {
        let new_dir = Dir {
            name,
            children: vec![],
            size: None,
        };
        self.children.push(Children::Dir(new_dir));
    }

    fn sum_size(&mut self) -> u32 {
        let sum = self.children.iter_mut().map(|child| {
            match child {
                Children::Dir(dir) => dir.sum_size(),
                Children::File(file) => file.size,
            }
        }).sum::<u32>();
        self.size = Some(sum);
        sum
    }

    // fn get_sizes(&self) -> &mut Vec<u32> {

    // }

    fn get_sizes_vec(&self) -> Vec<u32> {
        let mut sizes = self.children.iter().filter_map(|child| {
            match child {
                Children::Dir(dir) => {
                    let mut children = dir.get_sizes_vec();
                    children.append(&mut vec![dir.size.unwrap()]);
                    Some(children)
                },
                _ => None,
            }
        }).flatten().collect::<Vec<_>>();
        sizes.sort();
        sizes
    }

    fn get_result(&self) -> u32 {
        self.children.iter().map(|child| {
            match child {
                Children::Dir(dir) => {
                    let mut total_size = 0;
                    if let Some(size) = dir.size {
                        if size <= 100000 {
                            total_size += size
                        }
                        total_size += dir.get_result();
                        return total_size
                    }
                    0
                },
                _ => 0,
            }
        }).sum::<u32>()
    }
}

#[derive(Debug)]
enum Children {
    File(File),
    Dir(Dir),
}

pub fn calculate(s: String) -> u32 {
    let lines = s.lines();

    let mut root = Dir {
        name: "/".to_string(),
        children: vec![],
        size: None
    };
    let mut path = vec![];
    // let mut current: &mut Dir = &mut root;

    for line in lines.skip(1) {
        let commands: Vec<_> = line.split(' ').collect();
        let current = get_current(&mut root, path.clone());

        if commands.len() == 3 {
            // println!("change dir: {:?}", commands);
            let name = commands[2];
            if commands[2] != ".." {
                current.append_dir(name.to_string());
                path.push(name.to_string());
            } else {
                path.pop();
            }
        } else if commands[0] == "$" {
            // println!("ls dir: {:?}", commands);
        } else if commands[0] == "dir" {
            // println!("listed directory: {:?}", commands)
        } else {
            // println!("listed file: {:?}", commands)s
            current.children.push(Children::File(File {
                name: commands[1].to_string(),
                size: commands[0].parse().unwrap(),
            }))
        }
    }

    root.sum_size();
    root.get_result();
    let sizes_vec = root.get_sizes_vec();

    println!("{:?}", sizes_vec);
    *sizes_vec.iter().find(|item| 70000000 - root.size.unwrap() + *item >= 30000000).unwrap()
}

fn get_current(root: &mut Dir, path: Vec<String>) -> &mut Dir {
    let mut current = root;
    for name in path {
        let mut children_iter = current.children.iter_mut();
        current = children_iter
            .find_map(|item| match item {
                Children::Dir(dir) => {
                    if dir.name == *name {
                        Some(dir)
                    } else {
                        None
                    }
                }
                Children::File(_) => None,
            }).unwrap()
    }
    // println!("current: {:#?}", current);
    current
}

#[cfg(test)]
mod test {
    use super::calculate;

    #[test]
    fn base_case() {
        let input = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k".to_string();

        assert_eq!(calculate(input), 24933642);
    }
}
