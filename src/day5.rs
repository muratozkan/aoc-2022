use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve() {
    let file = File::open("in/day5.txt").unwrap();
    let moves: Vec<Move> = BufReader::new(file)
        .lines()
        .map(|l| parse_move(&l.unwrap()))
        .collect();

    /*
            [Q]     [G]     [M]    
            [B] [S] [V]     [P] [R]
    [T]     [C] [F] [L]     [V] [N]
[Q] [P]     [H] [N] [S]     [W] [C]
[F] [G] [B] [J] [B] [N]     [Z] [L]
[L] [Q] [Q] [Z] [M] [Q] [F] [G] [D]
[S] [Z] [M] [G] [H] [C] [C] [H] [Z]
[R] [N] [S] [T] [P] [P] [W] [Q] [G]
 1   2   3   4   5   6   7   8   9 
        */

    let stacks: Vec<Vec<char>> = [
        "RSLFQ", "NZQGPT", "SMQB", "TGZJHCBQ", "PHMBNFS", "PCQNSLVG", "WCF", "QHGZWVPM", "GZDLCNR",
    ]
    .iter()
    .map(|s| s.chars().collect::<Vec<char>>())
    .collect();

    part_one(&moves, stacks.clone());
    part_two(&moves, stacks.clone());
}

fn part_one(moves: &Vec<Move>, mut stacks: Vec<Vec<char>>) {
    for m in moves {
        for _ in 0..m.count {
            let c = stacks[m.from - 1].pop().unwrap();
            stacks[m.to - 1].push(c);
        }
    }

    let top: Vec<char> = stacks
        .iter()
        .map(|s: &Vec<char>| { s.last().unwrap().to_owned() } )
        .collect();


    println!("{}", String::from_iter(top.iter()));
}

fn part_two(moves: &Vec<Move>, mut stacks: Vec<Vec<char>>) {
    for m in moves {
        let mut temp: Vec<char> = Vec::new();
        for _ in 0..m.count {
            let c = stacks[m.from - 1].pop().unwrap();
            temp.push(c);
        }
        for _ in 0..m.count {
            let c = temp.pop().unwrap();
            stacks[m.to - 1].push(c);
        }
    }

    let top: Vec<char> = stacks
        .iter()
        .map(|s: &Vec<char>| { s.last().unwrap().to_owned() } )
        .collect();


    println!("{}", String::from_iter(top.iter()));

}

fn parse_move(l: &str) -> Move {
    let ms: Vec<&str> = l.split(" ").collect();
    Move {
        from: ms[3].parse().unwrap(),
        to: ms[5].parse().unwrap(),
        count: ms[1].parse().unwrap(),
    }
}

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    count: usize,
}
