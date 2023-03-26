fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let d1_ans = part1(&input);
    println!("Part 1: {}", d1_ans);
    let p2_ans = part2(&input);
    println!("part 2: {}", p2_ans)
}

fn part1(input: &String) -> i32 {
    input
        .lines()
        .map(|line| line.as_bytes())
        .map(|v| match (v[0], v[2]) {
            (b'A', b'X') => 4,
            (b'A', b'Y') => 8, // 2 + 6
            (b'A', b'Z') => 3, // 3 + 0
            (b'B', b'X') => 1, // 1 + 0
            (b'B', b'Y') => 5, // 2 + 3
            (b'B', b'Z') => 9, // 3 + 6
            (b'C', b'X') => 7, // 1 + 6
            (b'C', b'Y') => 2, // 2 + 0
            (b'C', b'Z') => 6,
            _ => panic!(),
        })
        .sum()
}

fn part2(input: &String) -> i32 {
    input
        .lines()
        .map(|line| line.as_bytes())
        .map(|v| match (v[0], v[2]) {
            (b'A', b'X') => 3,
            (b'A', b'Y') => 4, // 2 + 6
            (b'A', b'Z') => 8, // 3 + 0
            (b'B', b'X') => 1, // 1 + 0
            (b'B', b'Y') => 5, // 2 + 3
            (b'B', b'Z') => 9, // 3 + 6
            (b'C', b'X') => 2, // 1 + 6
            (b'C', b'Y') => 6, // 2 + 0
            (b'C', b'Z') => 7,
            _ => panic!(),
        })
        .sum()
}

// A = 1
// B = 2
// C = 3
