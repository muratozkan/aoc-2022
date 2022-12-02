use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve() {
    part_one();
    part_two();
}

fn part_one() {
    let file = File::open("in/day2.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let mut total: u64 = 0;
    for line in lines {
        let s = line.unwrap();
        let chars: Vec<char> = s.chars().collect();
        // other -> A: rock = 0, B: paper = 1; C: scissors = 2
        // my -> X: rock = 0, Y: paper = 1; Z: scissors = 2
        // A -> Y  0 1
        let my: u32 = (chars[2] as u32) - ('X' as u32);
        let other: u32 = (chars[0] as u32) - ('A' as u32);

        let round = score(my, other);
        total += round as u64;
        // println!("{} {}={}", my, other, round);
    }
    print!("Part 1: {}", total)
}

fn score(my: u32, other: u32) -> u32 {
    return if my == (other + 1) % 3 {
        // I win
        7 + my
    } else if other == (my + 1) % 3 {
        my + 1
    } else {
        4 + my
    }
}

fn part_two() {
    let file = File::open("in/day2.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let mut total: u64 = 0;
    for line in lines {
        let s = line.unwrap();
        let chars: Vec<char> = s.chars().collect();
        // other -> A: rock = 0, B: paper = 1; C: scissors = 2
        // outcome -> X: lose = 0, Y: draw = 1; Z: win = 2
        //  
        let outcome: u32 = (chars[2] as u32) - ('X' as u32);
        let other: u32 = (chars[0] as u32) - ('A' as u32);

        let my: u32 = match outcome {
            0 => (other + 3 - 1) % 3,
            1 => other,
            2 => (other + 1) % 3,
            _ => panic!("Should not happen")
        };

        let round = score(my, other);
        total += round as u64;
        // println!("{} {} ({}) = {}", other, outcome, my, round);
    }
    print!("Part 2: {}", total) 
}
