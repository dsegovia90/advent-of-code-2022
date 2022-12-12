
pub fn calculate(s: String) -> i32 {
    let lines = s.lines();

    let mut sprite_position = ['.'; 40];
    sprite_position[0] = '#';
    sprite_position[1] = '#';
    sprite_position[2] = '#';

    let mut register = vec![1];
    lines.for_each(|line| {
        register.push(*register.last().unwrap());
        if line.starts_with("addx") {
            let val = line.split(' ').last().unwrap().parse::<i32>().unwrap();
            register.push(*register.last().unwrap() + val);
        }
    });

    println!("{:?}", register);


    register.chunks(40).for_each(|pixels| {

        let mut row = ['.'; 40];

        pixels.iter().enumerate().for_each(|(index, register)| {
            if register > &0 && register < &39{
                let mut sprite_position = ['.'; 40];
                // println!("{register}");
                sprite_position[(register - 1) as usize] = '#';
                sprite_position[*register as usize] = '#';
                sprite_position[(register + 1) as usize] = '#';

                if sprite_position[index] == '#' {
                    row[index] = '#';
                }
            }
        });

        println!("{}", row.iter().collect::<String>())
    });

    0
}

#[cfg(test)]
mod test {
    use super::calculate;

    #[test]
    fn base_case() {
        let input = "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop".to_string();

        assert_eq!(calculate(input), 13140);
    }
}
