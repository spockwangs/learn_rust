use std::env;
use std::error::Error;
use std::fs;
use std::string::String;
use string_error::*;
use std::collections::HashSet;

struct Board {
    data: [[(i32, bool); 5]; 5],
}

impl Board {
    fn new() -> Self {
        Self {
            data: [[(0, false); 5]; 5],
        }
    }

    fn set(&mut self, i: usize, j: usize, num: i32) {
        self.data[i][j].0 = num;
    }

    fn mark(&mut self, val: i32) -> (bool, i32) {
        let mut all_true = false;
        for i in 0..5 {
            for j in 0..5 {
                if self.data[i][j].0 == val {
                    self.data[i][j].1 = true;

                    let mut row_true = true;
                    for k in 0..5 {
                        row_true &= self.data[i][k].1;
                    }
                    if row_true {
                        all_true = true;
                    } else {
                        row_true = true;
                        for k in 0..5 {
                            row_true &= self.data[k][j].1;
                        }
                        all_true = row_true;
                    }
                }
            }
        }
        
        let mut sum = 0;
        if all_true {
            for i in 0..5 {
                for j in 0..5 {
                    if self.data[i][j].1 == false {
                        sum += self.data[i][j].0;
                    }
                }
            }
        }
        return (all_true, sum);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        Err(into_err(format!("Usage: {} filename", &args[0])))?;
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("can't open file");
    let mut random_nums: Vec<i32> = vec![];
    let mut boards: Vec<Board> = vec![];
    let mut cur_board = Board::new();
    let mut cur_row = 0;
    for (index, line) in content.lines().enumerate() {
        if index == 0 {
            random_nums = line.split(',').map(|s| s.parse::<i32>()).collect::<Result<Vec<_>, _>>()?;
        } else if line.is_empty() {
            if index > 1 {
                boards.push(cur_board);
            }
            cur_board = Board::new();
            cur_row = 0;
        } else {
            let nums: Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>()).collect::<Result<Vec<_>, _>>()?;
            if nums.len() != 5 {
                Err("invalid number of numbers")?;
            }
            for i in 0..5 {
                cur_board.set(cur_row, i, nums[i]);
            }
            cur_row += 1;
        }
    }

    let mut winers = HashSet::new();
    for num in random_nums {
        for (index, j) in boards.iter_mut().enumerate() {
            if winers.contains(&index) {
                continue;
            }
            let (b, sum) = j.mark(num);
            if b {
                println!("{}", sum * num);
                winers.insert(index);
            }
        }
    }
    
    Ok(())
}
