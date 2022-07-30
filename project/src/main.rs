#![allow(unused_imports)]

pub mod linked_list;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use std::mem;
use std::thread;
use std::sync::mpsc::channel;
use std::rc::Rc;
use std::sync::Arc;
use std::borrow::BorrowMut;
use std::sync::atomic::{
    AtomicI8,
    Ordering,
};

pub mod raw_linked_list;
use raw_linked_list::List;

mod vec;

struct A {
}

impl Drop for A {
    fn drop(&mut self) {
        println!("dropping a");
    }
}

fn main() {
    let mut v = vec![1,2,3];
    println!("{}", v);
}
