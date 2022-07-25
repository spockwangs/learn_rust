#![allow(dead_code)]

use std::mem;
use std::ptr;
use std::alloc::Layout;
use std::alloc::alloc;
use std::alloc::dealloc;
use std::ops::Index;
use std::ops::IndexMut;

pub struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec {
            data: ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn push_back(&mut self, a: T) {
        if self.capacity < self.len + 1 {
            let new_capacity = if self.capacity == 0 {
                4
            } else {
                2 * self.capacity
            };
            self.reserve(new_capacity);
        }
        unsafe {
            let ptr = self.data.offset(self.len as isize);
            ptr::write(ptr, a);
        }
        self.len += 1;
    }

    pub fn reserve(&mut self, new_capacity: usize) {
        if new_capacity <= self.capacity {
            return;
        }

        let layout = Layout::array::<T>(new_capacity).unwrap_or_else(|_| panic!("capacity overflow"));
        let ptr;
        unsafe {
            ptr = alloc(layout) as *mut T;
            ptr::copy(self.data, ptr, self.len);
            let layout = Layout::array::<T>(self.capacity).unwrap_unchecked();
            dealloc(self.data as *mut u8, layout);
        }
        self.data = ptr;
        self.capacity = new_capacity;
    }

    pub fn shrink_to_fit(&mut self) {
        if self.capacity <= self.len {
            return;
        }

        let new_capacity = self.len;
        let ptr = if new_capacity > 0 {
            let layout = Layout::array::<T>(new_capacity).unwrap_or_else(|_| panic!("capacity overflow"));
            let p;
            unsafe {
                p = alloc(layout) as *mut T;
                ptr::copy(self.data, p, self.len);
            };
            p
        } else {
            ptr::null_mut()
        };
        unsafe {
            let layout = Layout::array::<T>(self.capacity).unwrap_unchecked();
            dealloc(self.data as *mut u8, layout);
        }
        self.data = ptr;
        self.capacity = new_capacity;
    }

    pub fn size(&self) -> usize {
        self.len
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            v: &self,
            cur: 0,
        }
    }
}

impl<T> Index<usize> for Vec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.size() {
            panic!("index out of bounds")
        } else {
            unsafe {
                &*self.data.offset(index as isize) as &T
            }
        }
    }
}

impl<T> IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.size() {
            panic!("index out of bounds")
        } else {
            unsafe {
                &mut *self.data.offset(index as isize) as &mut T
            }
        }
    }
}

impl<T> Clone for Vec<T>
where T: Clone {
    fn clone(&self) -> Self {
        let layout = Layout::array::<T>(self.len).unwrap_or_else(|_| panic!("capacity overflow"));
        let ptr;
        unsafe {
            ptr = alloc(layout) as *mut T;
            ptr::copy(self.data, ptr, self.len);
        }
        Self {
            data: ptr,
            len: self.len,
            capacity: self.len,
        }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.data, self.len));
            let layout = Layout::array::<T>(self.capacity).unwrap_unchecked();
            dealloc(self.data as *mut u8, layout);
        }
    }
}

pub struct Iter<'a, T> {
    v: &'a Vec<T>,
    cur: usize,
}

impl<'a, T> Iter<'a, T> {
    fn new(v: &'a Vec<T>) -> Self {
        Iter {
            v: v,
            cur: 0,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.v.size() {
            None
        } else {
            self.cur += 1;
            Some(&(self.v[self.cur-1]))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test() {
        let mut v = Vec::<i32>::new();
        assert_eq!(v.size(), 0);
        v.push_back(1);
        v.push_back(2);
        assert_eq!(v.size(), 2);
        assert_eq!(v[0], 1);
        assert_eq!(v[1], 2);
        v.push_back(3);
        v.push_back(4);
        v.push_back(5);
        v.push_back(6);
        assert_eq!(v[0], 1);
        assert_eq!(v[1], 2);
        v[5] = 25;
        assert_eq!(v[5], 25);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test2() {
        let mut v = Vec::<i32>::new();
        v[0] = 1;
    }
}