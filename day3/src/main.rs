fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("{}", part1(&input));
    println!("{}", part2(&input))
}

fn parse_input(input: &String) -> Vec<Vec<u8>>{
     input
        .lines()
        .map(|v| {
            v.bytes()
                .map(|v| match v {
                    b'a'..=b'z' => v - b'a' + 1,
                    b'A'..=b'Z' => v - b'A' + 27,
                    _ => 0,
                })
                .collect()
        })
        .collect()
}

fn part1(input: &String) -> i32 {
    let test: Vec<Vec<u8>> = input
        .lines()
        .map(|v| {
            v.bytes()
                .map(|v| match v {
                    b'a'..=b'z' => v - b'a' + 1,
                    b'A'..=b'Z' => v - b'A' + 27,
                    _ => 0,
                })
                .collect()
        })
        .collect();

    test.iter().map(|v| {
        let n = v.len() /2;
        let mut occupied = [false; 52];
        v[..n].iter().for_each(|&i| occupied[i as usize -1] = true);
        v[n..].iter().find(|&&i| occupied[i as usize -1]).unwrap()
    }).map(|&v| v as i32).sum::<i32>()
}


fn part2(input: &String) -> i32{
    parse_input(input)
    .chunks(3)
    .map(|v| {
            let mut occupied = [(false, false); 52];
            v[0].iter().for_each(|&i| occupied[i as usize -1 ].0 = true);
            v[1].iter().for_each(|&i| occupied[i as usize -1 ].1 = true);
            v[2].iter().find(|&&i| occupied[i as usize - 1] == (true, true)).unwrap()
            
        })
    .map(|&v| v as i32).sum()
}
