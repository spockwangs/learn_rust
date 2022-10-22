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
    let content = fs::read_to_string(filename).expect("can't open file");
    let mut total = 0;
    let v = content.lines().try_fold(vec![], |mut v, value| {
        total += 1;
        if v.is_empty() {
            v.resize(value.len(), 0);
        } else if v.len() != value.len() {
            Err(into_err(format!("bad input: {}", value)))?;
        }
        value.chars().enumerate().for_each(|(index, bit)| {
            if bit == '0' {
                v[index] += 1;
            }
        });
        Ok::<_, Box<dyn Error>>(v)
    }).unwrap();
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for e in v {
        if e <= total / 2 {
            gamma_rate = (gamma_rate << 1) + 1;
            epsilon_rate <<= 1;
        } else {
            gamma_rate <<= 1;
            epsilon_rate = (epsilon_rate << 1) + 1;
        }
    }
    println!("{}", gamma_rate * epsilon_rate);

    // Part 2
    let oxygen_gen_rating;
    let co2_rating;
    let (zero_bits, one_bits) : (Vec<_>, Vec<_>) = content.lines().partition(|line| {
        line.chars().nth(0).unwrap() == '0'
    });
    let (mut more_bits, mut less_bits) = {
        if zero_bits.len() >= one_bits.len() {
            (zero_bits, one_bits)
        } else {
            (one_bits, zero_bits)
        }
    };
    for i in 1.. {
        if more_bits.len() == 1 {
            break;
        }
        let (zero_bits, one_bits) : (Vec<_>, Vec<_>) = more_bits.into_iter().partition(|e| {
            e.chars().nth(i).unwrap() == '0'
        });
        if zero_bits.len() > one_bits.len() {
            more_bits = zero_bits;
        } else {
            more_bits = one_bits;
        }
    }
    oxygen_gen_rating = i32::from_str_radix(more_bits[0], 2).unwrap();
    for i in 1.. {
        if less_bits.len() == 1 {
            break;
        }
        let (zero_bits, one_bits) : (Vec<_>, Vec<_>) = less_bits.into_iter().partition(|e| {
            e.chars().nth(i).unwrap() == '0'
        });
        if zero_bits.len() <= one_bits.len() {
            less_bits = zero_bits;
        } else {
            less_bits = one_bits;
        }
    }
    co2_rating = i32::from_str_radix(less_bits[0], 2).unwrap();
    println!("{}", oxygen_gen_rating * co2_rating);
    
    Ok(())
}
