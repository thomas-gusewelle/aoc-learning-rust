fn main() {
    println!("{:?}", part1(&parse_input()))
}

fn parse_input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

// go through and find cds
// calc size of directories
//
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Dir {
    // parent: Dir,
    name: String,
    size: usize,
    files: Vec<i32>,
    dirs: Vec<Dir>,
}

impl Dir {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            size: 0,
            files: Vec::new(),
            dirs: Vec::new(),
        }
    }

    fn find_path(&mut self, path: &[Dir]) -> &mut Self {
        let mut current = self;
        for entry in path {
            current = current.dirs.iter_mut().find(|f| f.name == *entry.name).unwrap();
        }
        current
    }
}

fn part1(input: &String) {
    let mut fs = Dir::new("file_system");
    let mut current_dir = &fs; // keeps track of the current dir object
    let mut path: Vec<Dir> = Vec::new(); // keeps track of the order of dirs
    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts[..2] == ["$", "cd"] {
            match parts[2] {
                "/" => {
                    current_dir = &fs;
                    continue;
                }
                ".." => {
                    path.pop().unwrap();
                    continue;
                }
                _ => {}
             }
        } else {
            match parts[0] {
                "dir" => {
                    continue;
                }
                "$" => {} //$ ls
                _ => {}
            }
        }
    }
}
