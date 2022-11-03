use std::env;
use std::fs;
use std::process;
use std::collections::HashMap;
use std::sync::Mutex;
use std::str::FromStr;

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
    let graph = Graph::from_str(&content).expect("");
    println!("{}", graph.find_all_paths(1).len());

    // Part 2
    println!("{}", graph.find_all_paths(2).len());    
}

struct Graph {
    adjacent_array: HashMap<String, Vec<String>>,
}

impl FromStr for Graph {
    type Err = ParseError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = HashMap::<String, Vec<String>>::new();
        for line in s.trim().lines() {
            let splits = line.split('-').collect::<Vec<_>>();
            if splits.len() != 2 {
                return Err(ParseError::BadFormat);
            }
            if let Some(v) = result.get_mut(splits[0]) {
                v.push(splits[1].to_string());
            } else {
                result.insert(splits[0].to_string(), vec![splits[1].to_string()]);
            }
            if let Some(v) = result.get_mut(splits[1]) {
                v.push(splits[0].to_string());
            } else {
                result.insert(splits[1].to_string(), vec![splits[0].to_string()]);
            }
        }
        Ok(Graph { adjacent_array: result })
    }
}

#[derive(Debug)]
enum ParseError {
    BadFormat,
}

impl Graph {
    fn find_all_paths(&self, small_cave_max: i32) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        let ctx = Context::new(small_cave_max);
        self.dfs("start", ctx, &mut result);
        result
    }

    fn dfs(&self, start: &str, mut ctx: Context, paths: &mut Vec<Vec<String>>) {
        if let Some(v) = ctx.accessed.get_mut(start) {
            *v += 1;
            if Self::is_small_cave(start) {
                if *v > ctx.small_cave_max {
                    return;
                } else if *v == ctx.small_cave_max {
                    ctx.small_cave_max = 1;
                }
            }
        } else {
            ctx.accessed.insert(start.to_string(), 1);
        }
        ctx.path.push(start.to_string());
        if start == "end" {
            paths.push(ctx.path);
            return
        }

        for next in self.adjacent_array[start].iter() {
            if next == "start" {
                continue;
            }
            self.dfs(next, ctx.clone(), paths);
        }
    }

    fn is_small_cave(s: &str) -> bool {
        s.chars().nth(0).unwrap().is_ascii_lowercase()
    }
}

#[derive(Clone)]
struct Context {
    path: Vec<String>,
    accessed: HashMap<String, i32>,
    small_cave_max: i32,
}

impl Context {
    fn new(small_cave_max: i32) -> Self {
        Context {
            path: vec![],
            accessed: HashMap::new(),
            small_cave_max: small_cave_max,
        }
    }
}
