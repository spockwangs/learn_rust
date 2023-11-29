#![allow(unused_imports)]
#![feature(dropck_eyepatch)]

mod vec;

fn main() {
    let mut v = vec![1, 2];
    let a = v.pop().unwrap();
    let b = v.pop().unwrap();
    println!("{} {}", a, b);
}
