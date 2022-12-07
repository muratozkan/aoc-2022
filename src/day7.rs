use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve() {
    let file = File::open("in/day7.txt").unwrap();

    let message: Vec<String> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();

    let walker = Walker::new();
    for c in message {
        if c.starts_with("$ cd ") {
            walker.change_dir(&c[5..]);
            // println!("Path: {}", walker.get_path());
        } else if c != "$ ls" && !c.starts_with("dir") {
            let entry: Vec<&str> = c.split(" ").collect();
            walker.register_file(
                entry.get(0).unwrap().parse().unwrap(),
                entry.get(1).unwrap(),
            );
        }
    }

    let total: usize = walker
        .sizes
        .borrow()
        .iter()
        .map(|(_, s)| if *s <= 100000 { *s } else { 0 })
        .sum();

    println!("Part 1: {}", total);

    let capacity = 70000000;
    let unused = capacity - *walker.sizes.borrow().get("/").unwrap();
    let required = 30000000 - unused;

    let mut candidates: Vec<(String, usize)> = walker
        .sizes
        .borrow()
        .iter()
        .filter(|(_, s)| **s >= required)
        .map(|(p, s)| (p.clone(), s.clone()))
        .collect();

    candidates.sort_by_key(|p| p.1);
    let (path, size) = candidates.first().unwrap();

    println!("Part 2: {} {}", path, size);
}

struct Walker {
    dirs: RefCell<Vec<String>>,
    wd: RefCell<String>,
    sizes: RefCell<HashMap<String, usize>>,
}

impl Walker {
    fn new() -> Self {
        Walker {
            dirs: RefCell::new(Vec::new()),
            wd: RefCell::new(String::from("")),
            sizes: RefCell::new(HashMap::new()),
        }
    }

    fn make_path(path: &[String]) -> String {
        let s = RefCell::new(String::from(""));
        for p in path.iter() {
            s.borrow_mut().push_str(p);
            s.borrow_mut().push_str("/");
        }
        let rs = s.take();
        rs
    }

    fn change_dir(&self, ch: &str) {
        let mut ls = self.dirs.borrow_mut();
        if ch == "/" {
            ls.clear();
            ls.push(String::from(""));
        } else if ch == ".." {
            ls.pop();
        } else {
            ls.push(String::from(ch));
        }
        *self.wd.borrow_mut() = Walker::make_path(&ls);
    }

    fn register_file(&self, sz: usize, _name: &str) {
        let mut map = self.sizes.borrow_mut();
        let ls = self.dirs.borrow();
        // println!("file: {} size: {}", _name, sz);
        for i in (1..=ls.len()).rev() {
            let path = Walker::make_path(&ls[0..i]);
            let total = map.remove(&path).unwrap_or(0);
            // println!("i:{} p: {} total: {}", i, &path, total + sz);
            map.insert(path, total + sz);
        }
    }
}
