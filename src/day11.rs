use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve() {
    let file = File::open("in/day11.txt").unwrap();
    let ls: Vec<String> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();

    let mut monkeys: Vec<Monkey> = Vec::new();
    for i in 0..=(ls.len() / 7) {
        let ic = i * 7;
        let items: Vec<i64> = ls[ic + 1]
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        let ops = ls[ic + 2].strip_prefix("  Operation: new = old ").unwrap();
        let op_param = if ops.ends_with("old") {
            None
        } else {
            Some(ops[2..].parse::<i64>().unwrap())
        };
        let op = ops.chars().next().unwrap();
        let test: i64 = ls[ic + 3]
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse()
            .unwrap();
        let test_if: usize = ls[ic + 4]
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();
        let test_else: usize = ls[ic + 5]
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();
        monkeys.push(Monkey {
            op,
            op_param,
            test_div: test,
            test_if,
            test_else,
            items,
        })
    }
    println!("Part 1: {}", evaluate(&monkeys, 3, 20));
    println!("Part 2: {}", evaluate(&monkeys, 1, 10000));
}

fn evaluate(monkeys: &[Monkey], worry_factor: i64, rounds: i64) -> i64 {
    let mut lcm: i64 = 1;

    let mut hist: Vec<i64> = Vec::new();
    let mut monkey_items: Vec<Vec<i64>> = Vec::new();
    for m in monkeys.iter() {
        monkey_items.push(m.items.clone());
        hist.push(0);
        lcm *= m.test_div;
    }

    for i in 0..rounds {
        for (mi, monkey) in monkeys.iter().enumerate() {
            while !monkey_items[mi].is_empty() {
                hist[mi] += 1;

                let mut worry = monkey_items[mi].pop().unwrap() % lcm;
                worry = monkey.play(&worry) / worry_factor;
                let to = monkey.throw_to(&worry);

                monkey_items[to].push(worry);
            }
        }
        if i % 1000 == 0 {
            print_round_stats(&i, &monkey_items);
        }
    }

    println!("Totals: ");
    for (mi, v) in hist.iter().enumerate() {
        println!("  Monkey {}: {} times", mi, &v);
    }
    hist.sort_by(|a, b| b.cmp(a));
    hist[0] * hist[1]
}

fn print_round_stats(i: &i64, monkey_items: &[Vec<i64>]) {
    println!("Round {}", i);
    for (mi, m_items) in monkey_items.iter().enumerate() {
        print!("  Monkey {}: ", mi);
        for item in m_items.iter() {
            print!("{}, ", item);
        }
        println!();
    }
}

#[derive(Debug)]
struct Monkey {
    op: char,
    op_param: Option<i64>,
    test_div: i64,
    test_if: usize,
    test_else: usize,
    items: Vec<i64>,
}

impl Monkey {
    fn play(&self, item: &i64) -> i64 {
        match self.op_param {
            None => self.perform_op(item, item),
            Some(x) => self.perform_op(item, &x),
        }
    }

    fn perform_op(&self, p1: &i64, p2: &i64) -> i64 {
        match self.op {
            '+' => p1 + p2,
            '*' => p1 * p2,
            _ => panic!("Unknown op"),
        }
    }

    fn throw_to(&self, item: &i64) -> usize {
        if item % self.test_div == 0 {
            self.test_if
        } else {
            self.test_else
        }
    }
}
