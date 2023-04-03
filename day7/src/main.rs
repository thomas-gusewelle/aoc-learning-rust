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
    files: Vec<File>,
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

    fn find_parent(&mut self, path: &[String]) -> &mut Self {
        let mut current = self;
        println!("Current self: {:?}", current);
        for entry in path {
            current = current.dirs.iter_mut().find(|f| f.name == *entry).unwrap();
        }
        println!("find_path: {:?}", current);
        current
    }

    fn find_dir(&mut self, name: &str) -> &mut Self {
        println!("Current self: {:?}", self);
        self.dirs.iter_mut().find(|f| f.name == name).unwrap()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn new(name: &str, size: usize) -> Self {
        Self {
            name: name.to_string(),
            size: size,
        }
    }
}

fn part1(input: &String) {
    let mut fs = Dir::new("fs");
    // let mut current_dir = fs; // keeps track of the current dir object
    let mut current_dir: &str = "fs";
    let mut parent_dir: &str = "";
    let mut path_names: Vec<String> = Vec::new(); // keeps track of the order of dirs
    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts[..2] == ["$", "cd"] {
            match parts[2] {
                "/" => {
                    current_dir = "fs";
                    continue;
                }
                ".." => {
                    current_dir = parent_dir;
                    // path_names.pop().unwrap();
                    continue;
                }
                _ => {
                    parent_dir = current_dir;
                    current_dir = parts[2];
                    continue;
                }
            }
        }

        if parts[0] == "dir" {
            // make new dir
            let new_dir = Dir::new(parts[1]);
            // find partent dir
            let parent = fs.find_dir(current_dir);
            // push child dir into parent
            parent.dirs.push(new_dir);
            continue;
        }
        if let Ok(i) = parts[0].parse::<usize>() {
            let new_file = File::new(parts[1], i);
            fs.find_dir(current_dir).files.push(new_file);
        }
    }
}
