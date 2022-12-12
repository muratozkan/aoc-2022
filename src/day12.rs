use std::{
    fs::File,
    io::{BufRead, BufReader}, ops
};

pub fn solve() {
    let file = File::open("in/day12.txt").unwrap();
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect();

    let field = Field::from_input(&lines);

    let mut mins: Vec<Vec<i32>> = Vec::new();
    for _ in 0..field.size.y {
        let mut line: Vec<i32> = Vec::new();
        for _ in 0..field.size.x {
            line.push(-1);
        }
        mins.push(line);
    }

    mins = evaluate_paths(mins, &field, &field.target);
    println!("Part 1: {}", mins[field.start.y as usize][field.start.x as usize]);

    let mut min = i32::MAX;
    for yi in 0..field.size.y {
        for xi in 0..field.size.x {
            let pos = Pos2::new(xi, yi);
            if field.get_height(&pos) == 0 {
                let shortest = mins[pos.y as usize][pos.x as usize];
                if shortest != -1 && shortest < min {
                    min = shortest;
                }
            }
        }
    }
    println!("Part 2: {}", min);
}

fn evaluate_paths(mut mins: Vec<Vec<i32>>, field: &Field, start_pos: &Pos2) -> Vec<Vec<i32>> {
    let mut to_visit: Vec<Pos2> = Vec::new();
    to_visit.push(start_pos.clone());
    mins[start_pos.y as usize][start_pos.x as usize] = 0;
    while !to_visit.is_empty() {
        let current = to_visit.pop().unwrap();
        let current_min = mins[current.y as usize][current.x as usize];
        for d in DIRS {
            let next = &current + &d;
            if !field.in_bounds(&next) {
                continue;
            }
            if field.is_reachable_reverse(&current, &next) {
                let next_min = mins[next.y as usize][next.x as usize];
                if next_min == -1 || current_min + 1 < next_min {
                    mins[next.y as usize][next.x as usize] = current_min + 1;
                    to_visit.push(next);
                } 
            }
        }
    }
    mins
}

#[derive(Debug, Clone)]
struct Pos2 {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Field {
    height_map: Vec<Vec<u32>>,
    size: Pos2,
    start: Pos2,
    target: Pos2
}

impl Field {
    
    fn from_input(lines: &[String]) -> Self {
        let mut start = Pos2::zero();
        let mut target = Pos2::zero();
        let mut height_map: Vec<Vec<u32>> = Vec::new();

        for (yi, line) in lines.iter().enumerate() {
            let mut ln: Vec<u32> = Vec::new();
            for (xi, ch) in line.char_indices() { 
                let val = match ch {
                    'S' => { start = Pos2::new(xi as i32, yi as i32); 'a' }
                    'E' => { target = Pos2::new(xi as i32, yi as i32); 'z'  }
                    c => { c }
                } as u32 - ('a' as u32);
                ln.push(val);
            }
            height_map.push(ln);
        }
        let size = Pos2 { x: height_map[0].len() as i32, y: height_map.len() as i32 };
        Self {
            height_map,
            size,
            start,
            target
        }
    }

    fn in_bounds(&self, pos: &Pos2) -> bool {
        (pos.x >= 0 && pos.x < self.size.x) && (pos.y >= 0 && pos.y < self.size.y)
    }

    // from start to target
    fn _is_reachable(&self, current: &Pos2, next: &Pos2) -> bool {
        self.get_height(next) <= self.get_height(current) + 1 
    }

    // from target to start
    fn is_reachable_reverse(&self, current: &Pos2, next: &Pos2) -> bool {
        self.get_height(next) + 1 >= self.get_height(current)
    }

    fn get_height(&self, pos: &Pos2) -> u32 {
        self.height_map[pos.y as usize][pos.x as usize]
    }

}

const DIRS: [Pos2; 4] = [
    Pos2 { x: 1, y: 0}, // RIGHT
    Pos2 { x: 0, y: 1}, // DOWN
    Pos2 { x: -1, y: 0}, //LEFT
    Pos2 { x: 0, y: -1}  // UP
];

impl Pos2 {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    } 

    fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl ops::Add<&Pos2> for &Pos2 {
    type Output = Pos2;

    fn add(self, rhs: &Pos2) -> Self::Output {
        Pos2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl ops::Sub<&Pos2> for &Pos2 {
    type Output = Pos2;

    fn sub(self, rhs: &Pos2) -> Self::Output {
        Pos2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}
