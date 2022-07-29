#![allow(unused_imports)]
#![feature(dropck_eyepatch)]

pub mod linked_list;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use std::mem;
use std::marker::PhantomData;

pub mod raw_linked_list;
use raw_linked_list::List;
mod vec;

struct A<'a>(&'a i32);

/*
impl<'a> Drop for A<'a> {
    fn drop(&mut self) {
        println!("dropping a: {}", self.0);
    }
}
*/
fn main() {
    let mut v;
    let (i, a);
    i = 1;
    a = A(&i);
    v = vec::Vec::new();
        v.push(a);
    println!("{}", v.len());
}
