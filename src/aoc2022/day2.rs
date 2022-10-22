use std::process::ExitCode;
use std::env;
use std::error::Error;
use std::fs;
use std::string::String;
use string_error::*;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        Err(into_err(format!("Usage: {} filename", &args[0])))?;
    }

    // Part 1
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("can't open file");
    let position = contents.lines().try_fold::<_, _, Result<_, Box<dyn Error>>>((0, 0), |(h, d), value| {
        let splits: Vec<&str> = value.split(" ").collect();
        if splits.len() != 2 {
            Err(into_err(format!("bad input: {}", value)))?;
        }
        let num =  splits[1].parse::<i32>()?;
        if splits[0] == "forward" {
            Ok((h + num, d))
        } else if splits[0] == "down" {
            Ok((h, d + num))
        } else if splits[0] == "up" {
            Ok((h, d - num))
        } else {
            Err(into_err(format!("bad command: {}", splits[0])))?
        }
    }).unwrap();
    println!("{}", position.0 * position.1);

    // Part 2
    let position = contents.lines().try_fold::<_, _, Result<_, Box<dyn Error>>>((0, 0, 0), |(h, d, aim), value| {
        let splits: Vec<&str> = value.split(" ").collect();
        if splits.len() != 2 {
            Err(into_err(format!("bad input: {}", value)))?;
        }
        let num =  splits[1].parse::<i32>()?;
        if splits[0] == "forward" {
            Ok((h + num, d + aim*num, aim))
        } else if splits[0] == "down" {
            Ok((h, d, aim + num))
        } else if splits[0] == "up" {
            Ok((h, d, aim - num))
        } else {
            Err(into_err(format!("bad command: {}", splits[0])))?
        }
    }).unwrap();
    println!("{}", position.0 * position.1);

    Ok(())
}
