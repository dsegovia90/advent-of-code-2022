use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    // File hosts must exist in current path before this produces output

    if let Ok(lines) = read_lines("./input") {
        let mut current_max = 0;
        let mut max = vec![0, 0, 0];
        for line in lines.flatten() {
            if line.is_empty() {
                if max.clone().into_iter().any(|x| x < current_max) {
                    if max[0] < current_max {
                        max[0] = current_max;
                    } else if max[1] < current_max {
                        max[1] = current_max;
                    } else if max[2] < current_max {
                        max[2] = current_max;
                    }
                }
                current_max = 0;
                max.sort();
                println!("{:?}", max);
            } else {
                let num = line.parse::<i32>();
                if let Ok(num) = num {
                    current_max += num;
                }
            }
        }

        println!("{:?}", max.iter().sum::<i32>())
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;

    Ok(io::BufReader::new(file).lines())
}
