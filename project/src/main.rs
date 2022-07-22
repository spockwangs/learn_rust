#![allow(unused_imports)]

pub mod linked_list;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;

pub mod raw_linked_list;
use raw_linked_list::List;

fn main() {
    let mut l = List::<i32>::new();
    l.push_back(1);
    l.push_back(2);
    let mut it = l.iter();
    println!("{}", it.next().unwrap());

}
