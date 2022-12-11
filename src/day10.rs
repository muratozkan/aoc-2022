use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve() {
    let file = File::open("in/day10.txt").unwrap();
    let program: Vec<Instr> = BufReader::new(file)
        .lines()
        .map(|x| {
            let l = x.unwrap();
            match l.as_str() {
                s if s.starts_with("addx") => Instr::AddX {
                    x: l[5..].parse().unwrap(),
                },
                "noop" => Instr::Noop,
                _ => panic!("Unknown instr {}", l),
            }
        })
        .collect();

    part_one(&program);
    part_two(&program);
}

fn part_two(program: &[Instr]) {
    let mut x: i32 = 1;

    let mut ic: usize = 0;
    let mut current: &Instr = &program[0];
    let mut executing = current.cycles();
    for i in 1..241 {
        executing -= 1;

        let di = (i-1) % 40;
        if di == 0 {
            println!()
        }
        let pixel = if di >= x - 1 && di <= x + 1 {
            '#'
        } else {
            '.'
        };
        print!("{}", pixel);
        

        if executing == 0 {
            match *current {
                Instr::Noop => (),
                Instr::AddX { x: v } => { x += v; } ,
            }
            ic += 1;
            if ic < program.len() {
                current = &program[ic];
                executing = current.cycles();
            }
        }
        if executing == 0 {
            break;
        }
    }
}

fn part_one(program: &[Instr]) {
    let mut strength = 0;

    let mut x: i32 = 1;
    let mut ic: usize = 0;
    let mut current: &Instr = &program[0];
    let mut executing = current.cycles();
    for i in 1..221 {
        executing -= 1;
        // println!("C: {:?}, cyc: {}", &current, executing);
        if (i + 20) % 40 == 0 {
            strength += i * x;
            println!("{}\t x={} str={}", i, x, i*x);
        }
        if executing == 0 {
            match *current {
                Instr::Noop => (),
                Instr::AddX { x: v } => { x += v; } ,
            }
            ic += 1;
            if ic < program.len() {
                current = &program[ic];
                executing = current.cycles();
            }
        }
        if executing == 0 {
            break;
        }
    }

    println!("Part 1: {}", strength);
}

#[derive(Debug)]
enum Instr {
    Noop,
    AddX { x: i32 }
}

impl Instr {
    
    fn cycles(&self) -> i32 {
        match self {
            Self::Noop => 1,
            Self::AddX { x: _ } => 2
        }
    }
}