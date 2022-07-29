#![allow(unused_imports)]
#![feature(dropck_eyepatch)]

pub mod linked_list;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use std::mem;

pub mod raw_linked_list;
use raw_linked_list::List;
mod vec;

struct A<'a>(&'a i32);

impl<'a> Drop for A<'a> {
    fn drop(&mut self) {
        println!("dropping a: {}", self.0);
    }
}

fn f1() {
    let (a, i, mut b);
    i = 1;
    a = A{ 0: &i };
    b = std::vec::Vec::new();
    b.push(a);
    println!("{}", b.len());
}

fn main() {
    f1();
}
