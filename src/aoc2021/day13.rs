use std::env;
use std::fs;
use std::process;
use std::collections::HashSet;
use std::cmp;

#[macro_use]
extern crate lazy_static;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} filename", &args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("can't open file");

    let mut instruction_mode = false;
    let mut paper = Paper::new();
    let mut instructions: Vec<(char, i32)> = Vec::new();
    for line in content.trim().lines() {
        if line.trim().is_empty() {
            instruction_mode = true;
            continue;
        }
        if !instruction_mode {
            let splits = line.split(',').collect::<Vec<_>>();
            assert_eq!(splits.len(), 2);
            paper.insert((splits[0].parse::<i32>().unwrap(),
                          splits[1].parse::<i32>().unwrap()));
        } else {
            let splits = line.strip_prefix("fold along ").unwrap().split('=').collect::<Vec<_>>();
            assert_eq!(splits.len(), 2);
            if splits[0] == "x" {
                instructions.push(('x', splits[1].parse::<i32>().unwrap()));
            } else if splits[0] == "y" {
                instructions.push(('y', splits[1].parse::<i32>().unwrap()));
            } else {
                panic!("bad input");
            }
        }
    }

    for instruction in instructions {
        match instruction {
            ('x', n) => paper.fold_along_x(n),
            ('y', n) => paper.fold_along_y(n),
            _ => (),
        }
    }
    println!("{}", paper.num_of_dots());
    paper.print();
}

struct Paper {
    dots: HashSet<(i32, i32)>,
}

impl Paper {
    fn new() -> Self {
        Self {
            dots: HashSet::new(),
        }
    }

    fn insert(&mut self, point: (i32, i32)) {
        self.dots.insert(point);
    }
    
    fn fold<F>(&mut self, fold_dot: F, n: i32)
    where F: Fn((i32, i32), i32) -> (i32, i32) {
        let new_dots = self.dots.drain().map(|d| {
            fold_dot(d, n)
        }).collect();
        self.dots = new_dots;
    }

    fn fold_along_x(&mut self, n: i32) {
        self.fold(Self::fold_dot_along_x, n);
    }
    
    fn fold_along_y(&mut self, n: i32) {
        self.fold(Self::fold_dot_along_y, n);
    }

    fn fold_dot_along_x(mut d: (i32, i32), x: i32) -> (i32, i32) {
        if d.0 > x {
            d.0 = 2*x - d.0;
        }
        d
    }

    fn fold_dot_along_y(mut d: (i32, i32), y: i32) -> (i32, i32) {
        if d.1 > y {
            d.1 = 2*y - d.1;
        }
        d
    }

    fn num_of_dots(&self) -> usize {
        self.dots.len()
    }

    fn print(&self) {
        let (max_x, max_y) = self.dots.iter().fold((0, 0), |cur, &(x, y)| {
            (cmp::max(cur.0, x+1), cmp::max(cur.1, y+1))
        });
        for i in 0..max_x {
            for j in 0..max_y {
                if self.dots.contains(&(i, j)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}
