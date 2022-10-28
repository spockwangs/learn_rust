use std::env;
use std::fs;
use std::process;
use std::vec::Vec;
use std::cmp;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} filename", &args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("can't open file");
    let mut positions: Vec<i32> = content.trim().split(',').map(|s| s.parse::<i32>().expect("invalid digit")).collect();
    
    // Part 1
    positions.as_mut_slice().sort();
    let best_pos = positions[positions.len()/2];
    let fuels = positions.iter().fold(0, |acc, v| {
        acc + (v - best_pos).abs()
    });
    println!("{}", fuels);

    // Part 2
    let avg_pos = positions.iter().sum::<i32>() / positions.len() as i32;
    let best_idx1 = positions.iter().position(|&x| x >= avg_pos).unwrap();
    let best_pos1 = positions[best_idx1];
    let compute_fules = |pos: i32| -> i32 {
        positions.iter().fold(0, |acc, v| {
            let diff = (v - pos).abs();
            acc + diff * (diff + 1) / 2
        })
    };
    let mut best_fuels = compute_fules(best_pos1);
    if best_pos1 < avg_pos && positions.len() > best_idx1 + 1{
        let best_pos2 = positions[best_idx1+1];
        best_fuels = cmp::min(best_fuels, compute_fules(best_pos2));
    }
    println!("{}", best_fuels);
}
