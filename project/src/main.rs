#![allow(unused_imports)]
#![feature(dropck_eyepatch)]

pub mod linked_list;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use std::mem;
<<<<<<< HEAD
use std::thread;
use std::sync::mpsc::channel;
use std::rc::Rc;
use std::sync::Arc;
use std::borrow::BorrowMut;
use std::sync::atomic::{
    AtomicI8,
    Ordering,
};
=======
use std::marker::PhantomData;
>>>>>>> 2297ea96f39079a2ec0f9449c77ae7247b6021fd

pub mod raw_linked_list;
use raw_linked_list::List;
mod vec;

<<<<<<< HEAD
mod vec;

struct A {
}
=======
struct A<'a>(&'a i32);
>>>>>>> 2297ea96f39079a2ec0f9449c77ae7247b6021fd

/*
impl<'a> Drop for A<'a> {
    fn drop(&mut self) {
        println!("dropping a: {}", self.0);
    }
}
<<<<<<< HEAD

fn main() {
    let mut v = vec![1,2,3];
    println!("{}", v);
=======
*/
fn main() {
    let mut v;
    let (i, a);
    i = 1;
    a = A(&i);
    v = vec::Vec::new();
        v.push(a);
    println!("{}", v.len());
>>>>>>> 2297ea96f39079a2ec0f9449c77ae7247b6021fd
}
