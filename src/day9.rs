use std::{
    fs::File,
    io::{BufRead, BufReader}, collections::HashSet, cell::RefCell, ops
};

pub fn solve() {
    let file = File::open("in/day9.txt").unwrap();
    let map: Vec<(char, i32)> = BufReader::new(file)
        .lines()
        .map(|x| {
            let s = x.unwrap();
            let spl: Vec<&str> = s.split(' ').collect();
            (
                spl.first().unwrap().chars().next().unwrap(),
                spl.get(1).unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect();

    part_one(&map);
    part_two(&map, 10);
}

fn part_two(map: &[(char, i32)], t_count: usize) {
    let pos: RefCell<HashSet<Pos2>> = RefCell::new(HashSet::new());

    let mut ps: Vec<Pos2> = Vec::new();
    for _ in 0..t_count {
        ps.push(Pos2::zero());
    }

    pos.borrow_mut().insert(Pos2::zero());
    
    for (to, c) in map {
        let to = Pos2::from_dir(to);
        for _ in 0..*c {
            ps[0] = ps[0] + to;
            for i in 1..t_count {
                let mut t_pos = ps[i];

                let diff = ps[i - 1] - &t_pos;
                if diff.x.abs() >= 2 || diff.y.abs() >= 2 {
                    let dt_pos = Pos2::new(diff.x.signum(), diff.y.signum());
                    t_pos = t_pos + &dt_pos;
                    if i == t_count - 1 {
                        pos.borrow_mut().insert(t_pos);
                    }
                }

                ps[i] = t_pos;
                // println!("i={} H: {:?} T: {:?}", i, &ps[i - 1], t_pos);
            }
            // render(&ps);
        }
    }
    println!("Part 2: {}", pos.borrow().len());
}

fn part_one(map: &[(char, i32)]) {
    let pos: RefCell<HashSet<Pos2>> = RefCell::new(HashSet::new());

    let mut h_pos: Pos2 = Pos2::zero();
    let mut t_pos: Pos2 = Pos2::zero();

    pos.borrow_mut().insert(t_pos);
    
    for (to, c) in map {
        let to = Pos2::from_dir(to);
        for _ in 0..*c {
            let hn_pos = h_pos + to;
            let diff = hn_pos - &t_pos;
            if diff.x.abs() >= 2 || diff.y.abs() >= 2 {
                let dt_pos = Pos2::new(diff.x.signum(), diff.y.signum());
                // println!("T: {:?} H: {:?} --> delta: {:?}", &t_pos, &h_pos, &dt_pos);
                t_pos = t_pos + &dt_pos;
                pos.borrow_mut().insert(t_pos);
            }
            h_pos = hn_pos;
            // println!("H: {:?} T: {:?}", h_pos, t_pos);
            // render(&t_pos, &h_pos);
        }
    }
    println!("Part 1: {}", pos.borrow().len());
}

/*
fn render(v: &[Pos2]) {
    let mut map: HashMap<&Pos2, usize> = HashMap::new();
    for vi in (0..v.len()).rev() {
        map.insert(&v[vi], vi);
    }

    for y in -5..1 {
        for x in 0..6 {
            let p = Pos2::new(x, y);
            match p {
               x if map.contains_key(&x) => print!("{}", map[&x]),
               Pos2 { x: 0, y: 0 } => print!("s"),
               _ => print!(".")
            }
        }
        println!();
    }
    println!("===")
}
*/

const D_TO_POS: [Pos2; 4] = [
    Pos2{x: 0, y: -1},
    Pos2{x: 0, y: 1},
    Pos2{x: 1, y: 0},
    Pos2{x: -1, y: 0}
];

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Pos2 {
    x: i32,
    y: i32
}

impl Pos2 {
    
    fn from_dir(c: &char) -> &Self {
        match c {
            'U' => &D_TO_POS[0],
            'D' => &D_TO_POS[1],
            'R' => &D_TO_POS[2],
            'L' => &D_TO_POS[3],
            _ => panic!("Unknown pos")
        }
    }

    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    } 

    fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl ops::Add<&Pos2> for Pos2{
    type Output = Pos2;

    fn add(self, rhs: &Pos2) -> Self::Output {
        Pos2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl ops::Sub<&Pos2> for Pos2 {
    type Output = Pos2;

    fn sub(self, rhs: &Pos2) -> Self::Output {
        Pos2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}
