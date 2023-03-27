fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input))
}

fn parse_input(input: &String) -> Vec<[i32; 4]>{
    input.lines()
    .map(|v| {
            v.split(&['-', ','])
            .map(|v| v.parse().unwrap()).collect::<Vec<i32>>().try_into().unwrap()
        }).collect()
}

fn part1(input: &String) -> usize{
    parse_input(input)
    .iter().filter(|[a,b,c,d]| (a>=c && b<=d) || (c>=a && d<=b)).count()
}

fn part2(input: &String) -> usize {
    parse_input(input)
    .iter().filter(|[a,b,c,d]| (b>=c && a<=d) || (d>=a && c<=b)).count()
}
