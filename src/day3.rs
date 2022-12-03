use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve() {
    part_one();
    part_two();
}

fn part_one() {
    let file = File::open("in/day3.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let mut total_prio = 0;

    for l in lines {
        let ls = l.unwrap();
        let mid = ls.len() / 2;
        let common = find_common(&ls[0..mid], &ls[mid..]);
        let prio = common.iter().map(|c| priority(c)).nth(0).unwrap();
        // println!("Priority {} -> {} = {}", &ls, common.iter().nth(0).unwrap(), prio);

        total_prio += prio;
    }
    println!("Part 1: {}", total_prio);
}

fn part_two() {
    let file = File::open("in/day3.txt").unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let total_prio: i32 = lines
        .chunks_exact(3)
        .map(|c| {
            let c1 = find_common(&c[0], &c[1]);
            let c2 = find_common(&c[0], &c[2]);

            let common: HashSet<char> = c1.intersection(&c2).map(|c| c.clone()).collect();
            let prio = common.iter().map(|c| priority(c)).nth(0).unwrap();

            // println!("\n{}\n{}\n{}\n {} -> {}",&c[0],&c[1],&c[2],common.iter().nth(0).unwrap(),prio);

            prio
        })
        .sum();

    println!("Part 2: {}", total_prio);
}

fn find_common(s1: &str, s2: &str) -> HashSet<char> {
    let set1: HashSet<char> = s1.chars().map(|c| c.clone()).collect();
    let common = s2
        .chars()
        .filter(|c| set1.contains(c))
        .map(|c| c.clone())
        .collect();
    return common;
}

fn priority(c: &char) -> i32 {
    if c.is_uppercase() {
        *c as i32 - 'A' as i32 + 27
    } else {
        *c as i32 - 'a' as i32 + 1
    }
}
