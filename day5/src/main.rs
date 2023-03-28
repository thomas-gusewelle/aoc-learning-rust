fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    parse_input(&input)
}

struct Data {
    stacks: Vec<Vec<char>>,
    moves: Vec<[usize; 3]>,
}

fn parse_input(input: &String) -> Data {
    let mut stacks: Vec<Vec<char>> = vec![];
    let mut lines = input.lines();
    'stacks: for line in lines.by_ref() {
        let mut chars = line.trim_end().chars();
        let mut n = 0;
        // Building the structure of the stacks array based on the number of rows
        while let Some(c) = chars.nth(1) {
            if n >= stacks.len() {
                stacks.push(vec![]);
            }
            match c {
                'A'..='Z' => {
                    stacks[n].push(c);
                }
                '1'..='9' => {
                    break 'stacks;
                }
                ' ' => {}
                _ => {
                    panic!("I am panicking here");
                }
            }
            n += 1;
            chars.next();
            chars.next();
        }
    }
    for stack in stacks.iter_mut() {
        stack.reverse();
    }
    lines.next();

    // iterates over the moves lines and adds them to an array
    let mut moves: Vec<[usize; 3]> = vec![];
    for line in lines.by_ref() {
        let mut iter = line.split_whitespace();
        moves.push([
            iter.nth(1).unwrap().parse().unwrap(),
            iter.nth(1).unwrap().parse().unwrap(),
            iter.nth(1).unwrap().parse().unwrap(),
        ])
    }
    Data { stacks, moves }
}
