pub fn calculate(s: String) -> String {
    let lines: Vec<_> = s.lines().collect();
    let lines: Vec<_> = lines.split(|line| line.is_empty()).collect();

    let stacks_string: Vec<_> = lines[0].iter().copied().rev().collect();
    let moves_string = lines[1];
    let max = stacks_string
        .first()
        .unwrap()
        .trim()
        .split("   ")
        .map(|i| i.parse::<usize>().unwrap())
        .max().unwrap();

    let mut stacks_arr = vec![Vec::<char>::new(); max];

    for line in stacks_string.iter().skip(1) {
        let line: Vec<char> = line.replace(['[', ']'], " ").chars().collect();
        let chars: Vec<_> = line.chunks(4).map(|chunk| chunk[1]).collect();


        for (index, char) in chars.iter().enumerate() {
            if !char.is_whitespace() {
                stacks_arr[index].push(*char);
            }
        }
    }

    for line in moves_string {
        let line = line.replace("move ", "").replace(" from ", ",").replace(" to ", ",");
        let line: Vec<_> = line.split(',').map(|item| item.parse::<usize>().unwrap()).collect();

        let from = line[1] - 1;
        let to = line[2] - 1;
        let size = stacks_arr[from].len() - line[0];
        let mut move_slice: Vec<_> = stacks_arr[from].drain(size..).collect();
        stacks_arr[to].append(&mut move_slice);
    }


    let message: String = stacks_arr.iter().map(|vec| *vec.last().unwrap()).collect();
    message

}

#[cfg(test)]
mod test {
    use super::calculate;

    #[test]
    fn base_case() {
        let case = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2".to_string();

        let result = calculate(case);

        assert_eq!(result, "MCD".to_string())
    }
}
