#![allow(unused_imports)]

pub mod linked_list;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use std::mem;

pub mod raw_linked_list;
use raw_linked_list::List;

struct A {
}

impl Drop for A {
    fn drop(&mut self) {
        println!("dropping a");
    }
}

fn main() {
/*
    let mut l = List::<A>::new();
    l.push_back(A{});
*/
}
