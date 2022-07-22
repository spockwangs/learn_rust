use std::ptr;
use std::alloc::Layout;
use std::alloc::alloc;
extern crate alloc;
use alloc::alloc::dealloc;
use core::marker::PhantomData;

pub struct List<T> {
    head: RawLink<T>,
    tail: RawLink<T>,
}

struct RawLink<T> {
    ptr: *mut Node<T>,
}

impl<T> Clone for RawLink<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for RawLink<T> { }
    
impl<T> RawLink<T> {
    fn new(a: T) -> Self {
        unsafe {
            let layout = Layout::new::<Node<T>>();
            let ptr = alloc(layout) as *mut Node<T>;
            let r = &mut *ptr;
            r.element = a;
            r.next = RawLink::none();
            RawLink {
                ptr: ptr
            }
        }
    }

    fn none() -> Self {
        RawLink {
            ptr: ptr::null_mut(),
        }
    }

    fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    fn free(&mut self) {
        let layout = Layout::new::<Node<T>>();
        unsafe {
            dealloc(self.ptr as *mut u8, layout);
        }
    }
}

struct Node<T> {
    element: T,
    next: RawLink<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: RawLink::none(),
            tail: RawLink::none(),
        }
    }

    pub fn push_back(&mut self, a: T) {
        let node = RawLink::new(a);
        if self.head.is_null() {
            self.head = node;
            self.tail = node;
        } else {
            unsafe {
                (*self.tail.ptr).next = node;
            }
            self.tail = node;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.head.is_null() {
            None
        } else {
            let mut old_head = self.head;
            self.head = unsafe { (*old_head.ptr).next };
            if self.head.is_null() {
                self.tail = RawLink::none();
            }
            let r = unsafe {
                Some(ptr::read(old_head.ptr).element)
            };
            old_head.free();
            r
        }
    }

    pub fn front(&self) -> Option<&T> {
        if self.head.is_null() {
            None
        } else {
            unsafe {
                Some(&(*self.head.ptr).element)
            }
        }
    }

    pub fn iter(&self) -> ListIterator<'_, T> {
        ListIterator::new(&self)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() { }
    }
}

pub struct ListIterator<'a, T: 'a> {
    cur: RawLink<T>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.is_null() {
            None
        } else {
            let prev = self.cur;
            unsafe {
                self.cur = (*prev.ptr).next;
                Some(&(*prev.ptr).element)
            }
        }
    }
}

impl<'a, T> ListIterator<'a, T> {
    fn new(l: &List<T>) -> Self {
        ListIterator {
            cur: l.head,
            _marker: PhantomData,
        }
    }
}
    
