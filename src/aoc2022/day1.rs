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

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("can't open file");
    let mut count = 0;
    let mut previous = 0;
    contents.lines().enumerate().try_fold(0, |acc, (index, value)| {
        value.parse::<i32>().map(|num| {
            if index > 0 && acc < num {
                count += 1;
            }
            num
        })
    }).expect("invalid number");
    println!("count={}", count);
    return ExitCode::SUCCESS;
}
