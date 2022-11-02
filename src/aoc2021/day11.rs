use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;
use std::sync::Mutex;
use std::error::Error;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} filename", &args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("can't open file");

    // Part 1
    let mut grid = Grid::from_str(&content).expect("");
    let mut sum = 0;
    for _ in 0..100 {
        sum += grid.step();
    }
    println!("{sum}");

    // Part 2
    for i in 0.. {
        if grid.step() == 100 {
            println!("{}", i+101);
            break;
        }
    }
}

struct Grid {
    data: Vec<i32>,
}

impl FromStr for Grid {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result: Vec<i32> = vec![];
        for line in s.trim().lines() {
            let trimed_line = line.trim();
            if trimed_line.len() != 10 {
                return Err(ParseError::BadFormatError);
            }
            result.extend(
                trimed_line
                    .chars()
                    .map(|c| match c.to_digit(10) {
                        Some(d) => Ok(d as i32),
                        None => Err(ParseError::BadDigit),
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter(),
            );
        }
        if result.len() != 100 {
            return Err(ParseError::BadFormatError);
        }
        Ok(Grid { data: result })
    }
}

impl Grid {
    fn step(&mut self) -> i32 {
        for i in 0..self.data.len() {
            self.data[i] += 1;
            if self.data[i] == 10 {
                self.increase_adjacent(i);
            }
        }

        let mut flashes = 0;
        for e in self.data.iter_mut() {
            if *e > 9 {
                flashes += 1;
                *e = 0;
            }
        }
        flashes
    }

    fn increase_adjacent(&mut self, i: usize) {
        let (x, y) = self.to_coordinate(i);
        [
            (x - 1, y),
            (x + 1, y),
            (x, y - 1),
            (x, y + 1),
            (x - 1, y - 1),
            (x - 1, y + 1),
            (x + 1, y - 1),
            (x + 1, y + 1),
        ]
        .into_iter()
        .for_each(|(xx, yy)| {
            if let Some(j) = self.from_coordinate(xx, yy) {
                self.data[j] += 1;
                if self.data[j] == 10 {
                    self.increase_adjacent(j);
                }
            }
        });
    }

    fn to_coordinate(&self, i: usize) -> (i32, i32) {
        let i = i as i32;
        (i/10 as i32, i%10 as i32)
    }

    fn from_coordinate(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || x >= 10 || y < 0 || y >= 10 {
            return None;
        }

        Some((x*10 + y) as usize)
    }
}

#[derive(Debug)]
enum ParseError {
    BadFormatError,
    BadDigit,
}

// impl std::fmt::Display for ParseError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             ParseError::BadFormatError => write!(f, "BadFormatError"),
//             ParseError::BadDigit => write!(f, "BadDigit"),
//         }
//     }
// }

// impl Error for ParseError {}
