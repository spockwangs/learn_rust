use std::env;
use std::fs;
use std::process;
use std::vec::Vec;
use std::cmp;
use std::collections::HashSet;
use std::collections::HashMap;

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
    let heightmap = HeightMap::from(&content);

    // Part 1
    let total_risk_levels = heightmap.get_low_points().fold(0, |sum, e| {
        sum + e + 1
    });
    println!("{total_risk_levels}");

    // Part 2
    let mut size_of_basins: Vec<usize> = Vec::new();
    for idx in heightmap.get_low_points_idx() {
        size_of_basins.push(heightmap.get_basin_size(idx));
    }
    size_of_basins.sort_by(|a, b| b.cmp(a));
    println!("{}", size_of_basins.iter().take(3).product::<usize>());
}
        
struct HeightMap {
    data: Vec<i32>,
    num_rows: usize,
    num_cols: usize,
}

impl HeightMap {
    fn from(s: &str) -> Self {
        let mut result = Self {
            data: Vec::new(),
            num_rows: 0,
            num_cols: 0,
        };
        s.lines().for_each(|line| {
            let trimed = line.trim();
            result.num_cols = trimed.len();
            result.data.extend(trimed.chars().map(|x| x.to_digit(10).expect("bad digit") as i32));
            result.num_rows += 1;
        });

        result
    }

    fn get_low_points(&self) -> impl std::iter::Iterator<Item = i32> + '_ {
        self.get_low_points_idx().map(|idx| {
            self.data[idx]
        })
    }

    fn get_low_points_idx(&self) -> impl std::iter::Iterator<Item = usize> + '_ {
        self.data.iter().enumerate().filter(|&(idx, &x)| {
            if let Some(up) = self.up_height(idx) {
                if up <= x {
                    return false;
                }
            }
            if let Some(down) = self.down_height(idx) {
                if down <= x {
                    return false;
                }
            }
            if let Some(left) = self.left_height(idx) {
                if left <= x {
                    return false;
                }
            }
            if let Some(right) = self.right_height(idx) {
                if right <= x {
                    return false;
                }
            }
            return true;
        }).map(|(idx, _)| idx)
    }

    fn get_basin_size(&self, idx: usize) -> usize {
        let mut basin: HashSet<usize> = HashSet::new();

        fn mark(height_map: &HeightMap, index: usize, basin: &mut HashSet<usize>) {
            if basin.contains(&index) {
                return;
            }
            if height_map.data[index] >= 9 {
                return;
            }
            basin.insert(index);
            if let Some(index) = height_map.left(index) {
                mark(height_map, index, basin);
            }
            if let Some(index) = height_map.right(index) {
                mark(height_map, index, basin);
            }
            if let Some(index) = height_map.up(index) {
                mark(height_map, index, basin);
            }
            if let Some(index) = height_map.down(index) {
                mark(height_map, index, basin);
            }
        };
        mark(self, idx, &mut basin);
        basin.len()
    }

    fn left(&self, idx: usize) -> Option<usize> {
        let (i, j) = self.to_coordinate(idx);
        self.from_coordinate(i, j-1)
    }

    fn right(&self, idx: usize) -> Option<usize> {
        let (i, j) = self.to_coordinate(idx);
        self.from_coordinate(i, j+1)
    }

    fn up(&self, idx: usize) -> Option<usize> {
        let (i, j) = self.to_coordinate(idx);
        self.from_coordinate(i-1, j)
    }

    fn down(&self, idx: usize) -> Option<usize> {
        let (i, j) = self.to_coordinate(idx);
        self.from_coordinate(i+1, j)
    }

    fn up_height(&self, idx: usize) -> Option<i32> {
        let (i, j) = self.to_coordinate(idx);
        Some(self.data[self.from_coordinate(i-1, j)?])
    }

    fn down_height(&self, idx: usize) -> Option<i32> {
        let (i, j) = self.to_coordinate(idx);
        Some(self.data[self.from_coordinate(i+1, j)?])
    }

    fn left_height(&self, idx: usize) -> Option<i32> {
        let (i, j) = self.to_coordinate(idx);
        Some(self.data[self.from_coordinate(i, j-1)?])
    }

    fn right_height(&self, idx: usize) -> Option<i32> {
        let (i, j) = self.to_coordinate(idx);
        Some(self.data[self.from_coordinate(i, j+1)?])
    }

    fn to_coordinate(&self, idx: usize) -> (i32, i32) {
        ((idx/self.num_cols) as i32, (idx % self.num_cols) as i32)
    }

    fn from_coordinate(&self, i: i32, j: i32) -> Option<usize> {
        if i < 0 || (i as usize) >= self.num_rows || j < 0 || (j as usize) >= self.num_cols {
            None
        } else {
            let n = (i as usize *self.num_cols + j as usize) as usize;
            if n < self.data.len() {
                Some(n)
            } else {
                None
            }
        }
    }
}
