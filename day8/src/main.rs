fn main() {
    let input = std::fs::read_to_string("test-input.txt").unwrap();
    println!("Part 1: {:?}", part1(parse_input(&input)));
}

fn parse_input(input: &String) -> Vec<Vec<u32>> {
    let mut columns = vec![];
    for line in input.lines() {
        let row: Vec<u32> = line
            .chars()
            .into_iter()
            .map(|v| v.to_digit(10).unwrap())
            .collect();
        columns.push(row)
    }
    columns
}

fn part1(input: Vec<Vec<u32>>) -> usize {
    let mut num_visible = 0;
    num_visible += input.len() + input[0].len() - 2;
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            check_cord(x, y, &input, &mut num_visible);
        }
    }
    num_visible
}

fn check_cord(x: usize, y: usize, grid: &Vec<Vec<u32>>, total: &mut usize) {
    let cord_val: u32 = *grid.get(y).unwrap().get(x).unwrap();

    for j in 0..x {
        let num = grid[j][x];
        if j == x {
            continue;
        }
        if num >= cord_val {
            break;
        }
        *total += 1;
        return;
    }

    for j in x..grid.len() {
        let num = grid[j][y];
        if j == x {
            continue;
        }
        if num >= cord_val {
            break;
        }
        *total += 1;
        return;
    }

    for k in y..grid.len() {
        let num = grid[x][k];
        if k == y {
            continue;
        }
        if num >= cord_val {
            break;
        }
        *total += 1;
        return;
    }
    for k in 0..y {
        let num = grid[x][k];
        if k == y {
            continue;
        }
        if num >= cord_val {
            break;
        }
        *total += 1;
        return;
    }
}
