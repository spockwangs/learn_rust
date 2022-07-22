#![allow(unused_imports)]

use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: RawLink<T>,
}

type Link<T> = Option<Box<Node<T>>>;

type RawLink<T> = *mut Node<T>;

struct Node<T> {
    element: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(a: T) -> Box<Self> {
        Box::new(Node {
            element: a,
            next: None,
        })
    }
}

pub struct ListIterator<'a, T> {
    cur: Option<&'a Node<T>>,
}

impl<'a, T> ListIterator<'a, T> {
    fn new(l: &List<T>) -> ListIterator<T> {
        ListIterator {
            cur: l.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for ListIterator<'a, T>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur.map(|p| {
            self.cur = p.next.as_deref();
            &p.element
        })
    }
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List {
            head: None,
            tail: ptr::null_mut(),
        }
    }

    pub fn push_back(&mut self, a: T) {
        let mut x = Node::new(a);
        if self.head.is_none() {
            self.tail = &mut *x;
            self.head = Some(x);
        } else {
            let old_tail = self.tail;
            self.tail = &mut *x;
            unsafe {
                (*old_tail).next = Some(x);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T>
    {
        match self.head.take() {
            None => None,
            Some(old_head) => {
                self.head = old_head.next;
                if self.head.is_none() {
                    self.tail = ptr::null_mut();
                }
                Some(old_head.element)
            },
        }
    }

    pub fn front(&self) -> Option<&T>
    {
        self.head.as_ref().map(|head| &head.element)
    }

    pub fn back(&self) -> Option<&T> {
        if self.tail.is_null() {
            None
        } else {
            unsafe {
                Some(&(*self.tail).element)
            }
        }
    }

    pub fn iter(&self) -> ListIterator<T> {
        ListIterator::new(&self)
    }
}

impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for it in self.iter() {
            write!(f, "{}->", it)?;
        }
        write!(f, "()")
    }
}
