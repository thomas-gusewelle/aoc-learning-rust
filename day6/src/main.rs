fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("{:?}", parse_input(&input));
}

fn parse_input(input: &String) {
    input.into_iter().map(|v| {})
}
