use std::{
    collections::{HashMap, LinkedList},
    fs::File,
    io::{BufRead, BufReader}, cell::RefCell
};

pub fn solve() {
    let file = File::open("in/day6.txt").unwrap();
    
    let message: String = BufReader::new(file)
        .lines()
        .take(1)
        .map(|l| l.unwrap())
        .collect();
    
    // let message = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
    println!("Part 1: {}", start_of_message(4, &message.chars().collect()));
    println!("Part 2: {}", start_of_message(14, &message.chars().collect()));
}

struct Window<'a> {
    size: usize,
    list: RefCell<LinkedList<&'a char>>,
    histo: RefCell<HashMap<&'a char, usize>>,
}

impl <'a> Window<'a> {
    fn new(size: usize) -> Self {
        Window {
            size,
            list: RefCell::new(LinkedList::new()),
            histo: RefCell::new(HashMap::new()),
        }
    }

    fn insert(&'a self, ch: &'a char) {
        let mut ls = self.list.borrow_mut();
        let mut hs = self.histo.borrow_mut();

        if ls.len() == self.size {
            let front = ls.pop_front().unwrap();
            let count = hs.remove(front).unwrap();
            if count > 1 {
                hs.insert(front, count - 1);
            }
        }
        ls.push_back(ch);
        let count = hs.remove(ch).unwrap_or(0) + 1;
        hs.insert(ch, count);
    }

    fn num_distinct(&self) -> usize {
        self.histo.borrow().len()
    }

}

fn start_of_message(size: usize, message: &Vec<char>) -> usize {
    
    let window =  Box::new(Window::new(size));
    for ch in message[0..size].iter() {
        window.insert(ch);
    }

    let mut x: usize = 0;
    for i in size..message.len() {
        window.insert(message.get(i).unwrap());
        if window.num_distinct() == size {
            x = i;
            break;
        }
    }
    
    x + 1
}
