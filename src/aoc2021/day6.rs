use std::env;
use std::fs;
use std::process;
use std::collections::HashMap;
use std::sync::Mutex;

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

    // Part 1
    let ages: Vec<i32> = content.trim().split(',').map(|s| { s.parse::<i32>().expect("failed to parse int") }).collect();
    let mut total = 0;
    for a in ages.iter() {
        total += 1 + all_descendents(*a as u64, 80);
    }
    println!("{}", total);
    
    // Part 2
    total = 0;
    for a in ages.iter() {
        total += 1 + all_descendents(*a as u64, 256);
    }
    println!("{}", total);

}

fn direct_descendents(age: u64, n: u64) -> u64 {
    if n <= 0 || n < age+1 {
        0
    } else {
        (n - age - 1) / 7 + 1
    }
}

fn all_descendents(age: u64, n: u64) -> u64 {
    lazy_static! {
        static ref CACHE: Mutex<HashMap<(u64, u64), u64>> = Mutex::new(HashMap::new());
    }

    if let Some(v) = CACHE.lock().unwrap().get(&(age, n)) {
        return *v;
    }
    
    let mut result = direct_descendents(age, n);
    if n >= age + 1 {
        for i in 0..=(n-age-1)/7 {
            result += all_descendents(8, n - age - 1 - i*7);
        }
    }
    CACHE.lock().unwrap().insert((age, n), result);
    
    result
}
