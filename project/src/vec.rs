#![allow(dead_code)]

use std::alloc::Layout;
use std::alloc::alloc;
use std::alloc::dealloc;
use std::mem::ManuallyDrop;
use std::ops::Index;
use std::ops::IndexMut;
use std::ptr;
<<<<<<< HEAD
use std::fmt;
=======
use std::marker::PhantomData;
>>>>>>> 2297ea96f39079a2ec0f9449c77ae7247b6021fd

pub struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
    marker: PhantomData<T>,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec {
            data: ptr::null_mut(),
            len: 0,
            capacity: 0,
            marker: PhantomData,
        }
    }

    pub fn push(&mut self, a: T) {
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

    pub fn len(&self) -> usize {
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
        if index >= self.len() {
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
        if index >= self.len() {
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
            marker: PhantomData,
        }
    }
}

unsafe impl<#[may_dangle] T> Drop for Vec<T> {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.data, self.len));
            let layout = Layout::array::<T>(self.capacity).unwrap_unchecked();
            dealloc(self.data as *mut u8, layout);
        }
    }
}

#[macro_export]
macro_rules! vec {
    ( $($x:expr),* ) => {
        {
            let mut tmp_vec = vec::Vec::new();
            $(
                tmp_vec.push_back($x);
            )*
                tmp_vec
        }
    };
}

impl<T> IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut me = ManuallyDrop::new(self);
        IntoIter {
            start: me.data,
            capacity: me.capacity,
            cur: me.data,
            end: unsafe { me.data.add(me.len) },
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct IntoIter<T> {
    start: *const T,
    capacity: usize,
    cur: *const T,
    end: *const T,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == self.end {
            None
        } else {
            let old = self.cur;
            self.cur = unsafe { self.cur.add(1) };
            Some(unsafe {
                ptr::read(old)
            })
        }
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        unsafe {
            let len = self.end.offset_from(self.cur) as usize;
            if len > 0 {
                ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.cur as *mut T, len));
            }
            let layout = Layout::array::<T>(self.capacity).unwrap_unchecked();
            dealloc(self.start as *mut u8, layout);
        }
    }
}

impl<T> fmt::Display for Vec<T>
where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        for e in self {
            if !first {
                write!(f, ", ")?;
            } else {
                first = false;
            }
            write!(f, "{}", e)?;
        }
        write!(f, "]")
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
        if self.cur >= self.v.len() {
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
        assert_eq!(v.len(), 0);
        v.push(1);
        v.push(2);
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], 1);
        assert_eq!(v[1], 2);
        v.push(3);
        v.push(4);
        v.push(5);
        v.push(6);
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

    #[test]
    fn test3() {
        let mut v = Vec::<i32>::new();
        v.push(1);
        let mut it = v.iter();
        println!("{}", it.next().unwrap());
    }

    #[test]
    fn test4() {
        let mut v = Vec::<i32>::new();
        for i in 0..100 {
            v.push(i);
        }
        assert_eq!(v.len(), 100);
        let mut j = 0;
        for i in v {
            assert_eq!(i, j);
            j += 1;
        }
    }

    #[test]
    fn clone() {
        let mut v1 = Vec::<i32>::new();
        v1.push(2);
        v1.push(3);

        let v2 = v1.clone();
        assert_eq!(v1.len(), v2.len());
        for i in 0..v1.len() {
            assert_eq!(v1[i], v2[i]);
        }
    }
}
