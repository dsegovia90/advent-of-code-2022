use std::collections::VecDeque;

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u32>,
    operation: fn(u32) -> u32,
    throw_to_index: fn(u32) -> usize,
    inspects: u32,
}

pub fn calculate() -> u32 {
    let monkey0 = Monkey {
        items: VecDeque::from([89, 73, 66, 57, 64, 80]),
        inspects: 0,
        operation: |old| old * 3,
        throw_to_index: |item| {
            if item % 13 == 0 {
                return 6;
            }
            2
        },
    };
    let monkey1 = Monkey {
        items: VecDeque::from([83, 78, 81, 55, 81, 59, 69]),
        inspects: 0,
        operation: |old| old + 1,
        throw_to_index: |item| {
            if item % 3 == 0 {
                return 7;
            }
            4
        },
    };
    let monkey2 = Monkey {
        items: VecDeque::from([76, 91, 58, 85]),
        inspects: 0,
        operation: |old| old * 13,
        throw_to_index: |item| {
            if item % 7 == 0 {
                return 1;
            }

            4
        },
    };
    let monkey3 = Monkey {
        items: VecDeque::from([71, 72, 74, 76, 68]),
        inspects: 0,
        operation: |old| old * old,
        throw_to_index: |item| {
            if item % 2 == 0 {
                return 6;
            }

            0
        },
    };
    let monkey4 = Monkey {
        items: VecDeque::from([98, 85, 84]),
        inspects: 0,
        operation: |old| old + 7,
        throw_to_index: |item| {
            if item % 19 == 0 {
                return 5;
            }

            7
        },
    };
    let monkey5 = Monkey {
        items: VecDeque::from([78]),
        inspects: 0,
        operation: |old| old + 8,
        throw_to_index: |item| {
            if item % 5 == 0 {
                return 3;
            }

            0
        },
    };
    let monkey6 = Monkey {
        items: VecDeque::from([86, 70, 60, 88, 88, 78, 74, 83]),
        inspects: 0,
        operation: |old| old + 4,
        throw_to_index: |item| {
            if item % 11 == 0 {
                return 1;
            }

            2
        },
    };
    let monkey7 = Monkey {
        items: VecDeque::from([81, 58]),
        inspects: 0,
        operation: |old| old + 5,
        throw_to_index: |item| {
            if item % 16 == 0 {
                return 3;
            }

            5
        },
    };

    let mut monkeys = [monkey0, monkey1, monkey2, monkey3, monkey4, monkey5, monkey6, monkey7];

    for _ in 0..20 {
        for idx in 0..monkeys.len() {
            let monkey = monkeys.get_mut(idx).unwrap();
            let mut moves: Vec<(usize, u32)> = Vec::new();

            while !monkey.items.is_empty() {
                monkey.inspects += 1;
                let item = monkey.items.pop_front().unwrap();
                let item = (monkey.operation)(item) / 3;
                let monkey_index = (monkey.throw_to_index)(item);
                moves.push((monkey_index, item))
            }

            moves.iter().for_each(|(monkey_index, item)| {
                let monkey = monkeys.get_mut(*monkey_index).unwrap();
                monkey.items.push_back(*item)
            })
        }
    }

    monkeys.sort_by(|a, b| b.inspects.cmp(&a.inspects));

    monkeys.iter().take(2).fold(1, |acc, monkey| {
        monkey.inspects * acc
    })
}

#[cfg(test)]
mod test {
    use super::calculate;

    #[test]
    fn base_case() {
        assert_eq!(calculate(), 119715);
    }
}
