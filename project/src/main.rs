#![allow(unused_imports)]

pub mod linked_list;
use linked_list::List;
use std::ptr;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;

struct A {
}

impl Drop for A {
    fn drop(&mut self) {
        println!("dropping A");
    }
}

impl Display for A {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "A")
    }
}

fn main() {
    let mut l = List::<A>::new();
    l.push_back(A{});
    l.push_back(A{});
    let it = l.iter();
    drop(l);
    println!("a");
}
