fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
}

fn part1(input: &String) -> usize {
    input
        .as_bytes()
        .windows(4)
        .position(|v| !(0..3).any(|i| (i + 1..4).any(|j| v[i] == v[j])))
        .unwrap()
        + 4
}

fn part2(input: &String) -> usize {
    input
        .as_bytes()
        .windows(14)
        .position(|v| !(0..13).any(|i| (i + 1..14).any(|j| v[i] == v[j])))
        .unwrap()
        + 14
}
