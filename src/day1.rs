use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve() {
    let file = File::open("in/day1.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let mut nums: Vec<u64> = Vec::new();
    let mut sum: u64 = 0;
    for line in lines {
        let s = line.unwrap();
        if s.len() == 0 {
            nums.push(sum);
            sum = 0;
        } else { 
            let n:u64 = s.parse().unwrap();
            sum += n;
        }
    }
    nums.sort_unstable_by(|a, b| b.cmp(a));

    println!("Part 1: {}", nums[0]);
    println!("Part 2: {}", nums[0] + nums[1] + nums[2]);
}
