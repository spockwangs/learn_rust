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
    println!("{}", content.lines().map(|line| calculate_score(line)).sum::<i32>());

    // Part 2
    let mut scores: Vec<_> = content.lines().filter_map(|line| {
        let mut stack = Vec::new();
        for c in line.trim().chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    if let Some(a) = stack.pop() {
                        if is_matched(a, c) {
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                },
                _ => panic!("should not reach here"),
            }
        }
        let mut score = 0_i64;
        while let Some(a) = stack.pop() {
            score = score*5 + score_matched(a) as i64;
        }
        Some(score)
    }).collect();
    scores.sort();
    println!("{}", scores[scores.len()/2]);
}

fn calculate_score(s: &str) -> i32 {
    let mut stack = Vec::new();
    for c in s.trim().chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                if let Some(a) = stack.pop() {
                    if is_matched(a, c) {
                    } else {
                        return score(c);
                    }
                } else {
                    return score(c);
                }
            },
            _ => panic!("should not reach here"),
        }
    }
    return 0;
}

fn is_matched(l: char, r: char) -> bool {
    match l {
        '(' => r == ')',
        '[' => r == ']',
        '{' => r == '}',
        '<' => r == '>',
        _ => false,
    }
}

fn score(c: char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("bad char: {c}"),
    }
}

fn score_matched(c: char) -> i32 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("bad char: {c}"),
    }
}
