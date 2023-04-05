use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
fn main() {
    println!("{:?}", part1(&parse_input()))
}

fn parse_input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

// go through and find cds
// calc size of directories
//
#[derive(Default, Debug)]
struct Dir {
    parent: Option<Rc<Dir>>,
    size: RefCell<usize>,
    files: RefCell<Vec<File>>,
    dirs: RefCell<HashMap<String, Rc<Dir>>>,
}

impl Dir {
    // fn new(name: &str, parent_dir: Option<Rc<Dir>>) -> Self {
    //     Self {
    //         parent: parent_dir,
    //         name: name.to_string(),
    //         size: 0,
    //         files: Vec::new(),
    //         dirs: Vec::new(),
    //     }
    // }
    fn new() -> Self {
        Self::default()
    }

    // fn find_parent(&mut self, path: &[String]) -> &mut Self {
    //     let mut current = self;
    //     for entry in path {
    //         current = current.dirs.iter_mut().find(|f| f.name == *entry).unwrap();
    //     }
    //     current
    // }
    //
    // fn calc_dir_sizes(fs: &Dir) -> usize {
    //     let mut size = 0;
    //     size += fs.files.iter().map(|s| s.size).sum::<usize>();
    //     size
    // }
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
            size,
        }
    }
}

fn part1(input: &String) {
    let fs = Rc::new(Dir::new());
    // let mut path_names: Vec<String> = Vec::new();
    let mut current_dir = Rc::clone(&fs);
    // let mut sizes: Vec<usize> = Vec::new();
    // keeps track of the order of dirs
    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts[..2] == ["$", "cd"] {
            match parts[2] {
                // root
                "/" => {
                    continue;
                }
                // up dir
                ".." => {
                    current_dir = Rc::clone(&current_dir.parent.as_ref().unwrap());
                    // path_names.pop().unwrap();
                    continue;
                }
                // cd into dir
                dir_name => {
                    let move_dir = current_dir.dirs.borrow().get(dir_name).unwrap().clone();
                    current_dir = move_dir;
                }
            }
        }
        if parts[..2] == ["$", "ls"] {
            continue;
        }

        if parts[0] == "dir" {
            // make new dir
            let name = parts[1];
            let new_dir = Rc::new(Dir {
                parent: Some(Rc::clone(&current_dir)),
                size: RefCell::new(0),
                files: RefCell::new(vec![]),
                dirs: RefCell::new(HashMap::new()),
            });
            current_dir
                .dirs
                .borrow_mut()
                .insert(name.to_string(), new_dir);
            continue;
        }
        if let Ok(i) = parts[0].parse::<usize>() {
            let new_file = File::new(parts[1], i);
            *current_dir.size.borrow_mut() += i;
            current_dir.files.borrow_mut().push(new_file)
        }
    }
    println!("{:?}", fs)
}
