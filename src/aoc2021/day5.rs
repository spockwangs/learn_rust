use std::cmp::{max, min};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::hash::Hash;
use std::str::FromStr;
use std::string::String;
use string_error::*;

#[derive(Debug)]
enum ParseError {
    BadLenError,
    ParseDigitError(std::num::ParseIntError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::BadLenError => write!(f, "BadLenError"),
            ParseError::ParseDigitError(e) => write!(f, "ParseDigitError: {:?}", e),
        }
    }
}

impl Error for ParseError {}

impl From<std::num::ParseIntError> for ParseError {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::ParseDigitError(e)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LineSegment {
    p1: Point,
    p2: Point,
}

impl FromStr for LineSegment {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<&str> = s.split("->").collect();
        if fields.len() != 2 {
            return Err(ParseError::BadLenError);
        }
        let p1 = fields[0].parse()?;
        let p2 = fields[1].parse()?;
        Ok(LineSegment { p1: p1, p2: p2 })
    }
}

impl LineSegment {
    fn gen_points(&self) -> Vec<Point> {
        let mut result = vec![];
        let mut left = &self.p1;
        let mut right = &self.p2;
        if left.x > right.x {
            std::mem::swap(&mut left, &mut right);
        }
        if left.x == right.x {
            for y in min(left.y, right.y)..=max(left.y, right.y) {
                result.push(Point { x: left.x, y: y });
            }
        } else {
            let mut cur_y = left.y;
            let dir = right.y - left.y;
            for x in left.x..=right.x {
                result.push(Point { x: x, y: cur_y });
                match dir {
                    d if d > 0 => cur_y += 1,
                    d if d < 0 => cur_y -= 1,
                    _ => (),
                }
            }
        }
        result
    }

    fn is_horizontal(&self) -> bool {
        if self.p1.y == self.p2.y {
            true
        } else {
            false
        }
    }

    fn is_vertical(&self) -> bool {
        if self.p1.x == self.p2.x {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.trim().split(',').collect();
        if v.len() != 2 {
            return Err(ParseError::BadLenError);
        }
        let x = v[0].parse::<i32>()?;
        let y = v[1].parse::<i32>()?;
        Ok(Point { x: x, y: y })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        Err(into_err(format!("Usage: {} filename", &args[0])))?;
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("can't open file");
    let line_segments: Vec<LineSegment> = content
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<_>, _>>()?;

    // Part 1
    let mut points_map = HashMap::new();
    for ls in line_segments.iter() {
        if !ls.is_horizontal() && !ls.is_vertical() {
            continue;
        }
        for p in ls.gen_points() {
            if let Some(cnt) = points_map.get_mut(&p) {
                *cnt += 1;
            } else {
                points_map.insert(p, 1);
            }
        }
    }
    let num_of_intersections = points_map.iter().filter(|&(_, v)| *v >= 2).count();
    println!("{}", num_of_intersections);

    // Part 2
    let mut points_map = HashMap::new();
    for ls in line_segments {
        for p in ls.gen_points() {
            if let Some(cnt) = points_map.get_mut(&p) {
                *cnt += 1;
            } else {
                points_map.insert(p, 1);
            }
        }
    }
    let num_of_intersections = points_map.iter().filter(|&(_, v)| *v >= 2).count();
    println!("{}", num_of_intersections);

    Ok(())
}
