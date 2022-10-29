use std::env;
use std::fs;
use std::process;
use std::vec::Vec;
use std::cmp;
use std::collections::HashSet;
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

type InputOutput<'a> = (Vec<&'a str>, Vec<&'a str>);

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} filename", &args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("can't open file");
    let mut input_outputs: Vec<InputOutput> = Vec::new();
    content.lines().for_each(|line| {
        let splits: Vec<&str> = line.split('|').collect();
        if splits.len() != 2 {
            eprintln!("bad len");
            process::exit(1);
        }
        let in_out = (splits[0].trim().split(' ').collect(), splits[1].trim().split(' ').collect());
        input_outputs.push(in_out);
    });

    // Part 1
    let mut one_cnts = 0;
    let mut four_cnts = 0;
    let mut seven_cnts = 0;
    let mut eight_cnts = 0;
    for e in input_outputs.iter() {
        for s in e.1.iter() {
            match s.len() {
                2 => one_cnts += 1,
                4 => four_cnts += 1,
                3 => seven_cnts += 1,
                7 => eight_cnts += 1,
                _ => (),
            }
        }
    }
    println!("{}", one_cnts + four_cnts + seven_cnts + eight_cnts);

    // Part 2
    let mut sum = 0;
    for e in input_outputs.iter() {
        let connections = Connections::guess_from(&e.0);
        let mut num = 0;
        for s in e.1.iter() {
            let n = connections.decode(s);
            num = num*10 + n;
        }
        sum += num;
    }
    println!("{}", sum);
}

struct Connections {
    mapping: [char; 7],
}

impl Connections {
    fn record_map(&mut self, a: &char, b: &char) {
        self.mapping[*a as usize - 'a' as usize] = *b;
    }
    
    fn guess_from(inputs: &Vec<&str>) -> Self {
        let mut one_set: HashSet<char> = HashSet::new();
        let mut four_set: HashSet<char> = HashSet::new();
        let mut seven_set: HashSet<char> = HashSet::new();
        let mut eight_set: HashSet<char> = HashSet::new();
        let mut five_len_sets: Vec<HashSet<char>> = Vec::new();
        inputs.iter().for_each(|s| {
            match s.len() {
                2 => one_set = HashSet::from_iter(s.chars()),
                4 => four_set = HashSet::from_iter(s.chars()),
                3 => seven_set = HashSet::from_iter(s.chars()),
                7 => eight_set = HashSet::from_iter(s.chars()),
                5 => five_len_sets.push(HashSet::from_iter(s.chars())),
                _ => (),
            }
        });

        let mut result = Self {
            mapping: ['a'; 7],
        };
        // 7 - 1 => a
        let a: HashSet<_> = seven_set.difference(&one_set).copied().collect();
        assert_eq!(a.len(), 1);
        result.mapping[0] = *a.iter().nth(0).unwrap();
        // 1 ^ (5) => 3
        let mut three_set: HashSet<char> = HashSet::new();
        let mut two_set: HashSet<char> = HashSet::new();
        let mut five_set: HashSet<char> = HashSet::new();
        for e in five_len_sets {
            if one_set.intersection(&e).count() == 2 {
                three_set = e;
            } else {
                match four_set.intersection(&e).count() {
                    2 => two_set = e,
                    3 => five_set = e,
                    _ => (),
                }
            }
        }
        // 1 ^ 5 => f
        let f: HashSet<_> = one_set.intersection(&five_set).copied().collect();
        assert_eq!(f.len(), 1);
        result.record_map(f.iter().nth(0).unwrap(), &'f');
        // 1 - f => c
        let c: HashSet<_> = one_set.difference(&f).copied().collect();
        assert_eq!(c.len(), 1);
        result.record_map(c.iter().nth(0).unwrap(), &'c');
        // 3 - a - 4 => g
        let g: HashSet<_> = three_set.difference(&a).copied().collect::<HashSet<_>>()
            .difference(&four_set).copied().collect();
        assert_eq!(g.len(), 1);
        result.record_map(g.iter().nth(0).unwrap(), &'g');
        // 3 - a - c - f - g => d
        let d: HashSet<_> = three_set.difference(&a).copied().collect::<HashSet<_>>()
            .difference(&c).copied().collect::<HashSet<_>>()
            .difference(&f).copied().collect::<HashSet<_>>()
            .difference(&g).copied().collect::<HashSet<_>>();
        assert_eq!(d.len(), 1);
        result.record_map(d.iter().nth(0).unwrap(), &'d');
        // 4 - c - d - f => b
        let b: HashSet<_> = four_set.difference(&c).copied().collect::<HashSet<_>>()
            .difference(&d).copied().collect::<HashSet<_>>()
            .difference(&f).copied().collect();
        assert_eq!(b.len(), 1);
        result.record_map(b.iter().nth(0).unwrap(), &'b');
        // 2 - a - c - d - g => e
        let e: HashSet<_> = two_set.difference(&a).copied().collect::<HashSet<_>>()
            .difference(&c).copied().collect::<HashSet<_>>()
            .difference(&d).copied().collect::<HashSet<_>>()
            .difference(&g).copied().collect();
        assert_eq!(e.len(), 1);
        result.record_map(e.iter().nth(0).unwrap(), &'e');
        
        result
    }

    fn decode(&self, s: &str) -> i32 {
        lazy_static! {
            static ref DIGIT_MAP: HashMap<&'static str, i32> = HashMap::from([
                ("abcefg", 0),
                ("cf", 1),
                ("acdeg", 2),
                ("acdfg", 3),
                ("bcdf", 4),
                ("abdfg", 5),
                ("abdefg", 6),
                ("acf", 7),
                ("abcdefg", 8),
                ("abcdfg", 9),
            ]);
        }

        let mut mapped_s: Vec<char> = s.chars().map(|c| self.mapping[c as usize - 'a' as usize]).collect();
        mapped_s.sort();
        let ss: String = mapped_s.into_iter().collect();
        *DIGIT_MAP.get(&ss[..]).unwrap()
    }
}
