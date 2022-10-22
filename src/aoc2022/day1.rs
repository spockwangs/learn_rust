use std::process::ExitCode;
use std::env;
use std::error::Error;
use std::fs;
use std::string::String;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} filename", &args[0]);
        return ExitCode::FAILURE;
    }

    // Part 1
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("can't open file");
    let mut count = 0;
    contents.lines().enumerate().try_fold(0, |acc, (index, value)| {
        value.parse::<i32>().map(|num| {
            if index > 0 && acc < num {
                count += 1;
            }
            num
        })
    }).expect("invalid number");
    println!("count={}", count);

    // Part 2
    count = 0;
    contents.lines().enumerate().try_fold([0; 3], |window, (index, value)| {
        value.parse::<i32>().map(|num| {
            let new_window = [window[1], window[2], num];
            if index >= 3 {
                let old_sum: i32 = window.iter().sum();
                let new_sum = new_window.iter().sum();
                if old_sum < new_sum {
                    count += 1;
                }
            }
            new_window
        })
    }).expect("invalid number");
    println!("count={}", count);
                
    return ExitCode::SUCCESS;
}
