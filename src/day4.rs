use std::{fs::File, io::{BufReader, BufRead}};



pub fn solve() {
    let file = File::open("in/day4.txt").unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let mut c_count = 0;
    let mut o_count = 0;
    for line in lines {
        let (e1, e2) = parse(&line);
        let contains = contains(&e1, &e2);
        if contains {
            c_count += 1;
            //println!("{:?} <-c-> {:?}", &e1, &e2);
        }
        if overlaps(&e1, &e2) {
            o_count += 1;
            // if !contains {
            //    println!("{:?} <-o-> {:?}", &e1, &e2);
            // }
        }

    }
    println!("Part 1: {}", c_count);
    println!("Part 2: {}", o_count);

}

fn overlaps(e1: &(u32, u32), e2: &(u32, u32)) -> bool {
    (e2.0 >= e1.0 && e2.0 <= e1.1) || (e1.0 >= e2.0 && e1.0 <= e2.1)
}

fn contains(e1: &(u32, u32), e2: &(u32, u32)) -> bool {
    (e1.0 >= e2.0 && e1.1 <= e2.1) || (e2.0 >= e1.0 && e2.1 <= e1.1)
}

fn parse(line: &String) -> ((u32, u32), (u32, u32)) {
    let split: Vec<&str> = line.split(",").collect();
    let e1v: Vec<&str> = split[0].split("-").collect();
    let e1: (u32, u32) = (
        e1v[0].parse().unwrap(),
        e1v[1].parse().unwrap()
    );
    
    let e2v: Vec<&str> = split[1].split("-").collect();
    let e2: (u32, u32) = (
        e2v[0].parse().unwrap(),
        e2v[1].parse().unwrap()
    );

    (e1, e2)
}
