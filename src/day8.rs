use std::{
    cell::RefCell,
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve() {
    let file = File::open("in/day8.txt").unwrap();

    let map: Vec<Vec<u32>> = BufReader::new(file)
        .lines()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let sz = (map.get(0).unwrap().len() as i32, map.len() as i32);

    part_one(&sz, &map);
    part_two(&sz, &map);
}

fn part_two(sz: &(i32, i32), map: &Vec<Vec<u32>>) {
    let mut max_score = 0;

    for ix in 1..(sz.0 - 1) {
        for iy in 1..(sz.1 - 1) {
            let tree = map.get(iy as usize).unwrap().get(ix as usize).unwrap();
            println!("({},{}) tree={}", ix, iy, tree);
            // north
            let mut c_n = 0;
            for jy in (iy+1)..sz.1 {
                let h = map.get(jy as usize).unwrap().get(ix as usize).unwrap();
                if *h >= *tree {
                    c_n += 1;
                    break;
                }
                // println!("    North: -> {} @ {}", jy, h);
                c_n += 1;
            }
            // south
            let mut c_s = 0;
            for jy in (0..iy).rev() {
                let h = map.get(jy as usize).unwrap().get(ix as usize).unwrap();
                if *h >= *tree {
                    c_s += 1;
                    break;
                }
                // println!("    South: -> {} @ {}", jy, h);
                c_s += 1;
            }

            println!("    West: {:?}", ((ix+1)..sz.0));
            // west
            let mut c_w = 0;
            for jx in (ix+1)..sz.0 {
                let h = map.get(iy as usize).unwrap().get(jx as usize).unwrap();
                if *h >= *tree {
                    c_w += 1;
                    break;
                }
                // println!("    West: -> {} @ {}", jx, h);
                c_w += 1;
            }

            //east
            println!("    East: {:?}", (0..ix).rev());
            let mut c_e = 0;
            for jx in (0..ix).rev() {
                let h = map.get(iy as usize).unwrap().get(jx as usize).unwrap();
                if *h >= *tree {
                    c_e += 1;
                    break;
                }
                // println!("    East: -> {} @ {}", jx, h);
                c_e += 1;
            }

            let score = c_e * c_n * c_s * c_w;
            println!("({},{}) score={}", ix, iy, score);
            if score > max_score {
                max_score = score;
            }
        }
    }

    println!("Part 2: {}", max_score);
}

fn part_one(sz: &(i32, i32), map: &Vec<Vec<u32>>) {
    let visible: RefCell<HashSet<(i32, i32)>> = RefCell::new(HashSet::new());

    // north-south
    for ix in 0..sz.0 {
        let mut prev: i32 = -1;
        for iy in 0..sz.1 {
            let c = map.get(iy as usize).unwrap().get(ix as usize).unwrap();
            if *c as i32 > prev {
                visible.borrow_mut().insert((ix, iy));
                prev = *c as i32;
            }
        }
    }

    // south-north
    for ix in 0..sz.0 {
        let mut prev: i32 = -1;
        for iy in (0..sz.1).rev() {
            let c = map.get(iy as usize).unwrap().get(ix as usize).unwrap();
            if *c as i32 > prev {
                visible.borrow_mut().insert((ix, iy));
                prev = *c as i32;
            }
        }
    }

    // east-west
    for iy in 0..sz.1 {
        let mut prev: i32 = -1;
        for ix in 0..sz.0 {
            let c = map.get(iy as usize).unwrap().get(ix as usize).unwrap();
            if *c as i32 > prev {
                visible.borrow_mut().insert((ix, iy));
                prev = *c as i32;
            }
        }
    }

    // west-east
    for iy in 0..sz.1 {
        let mut prev: i32 = -1;
        for ix in (0..sz.0).rev() {
            let c = map.get(iy as usize).unwrap().get(ix as usize).unwrap();
            if *c as i32 > prev {
                visible.borrow_mut().insert((ix, iy));
                prev = *c as i32;
            }
        }
    }

    println!("Part 1: {}", visible.borrow().len());
}
