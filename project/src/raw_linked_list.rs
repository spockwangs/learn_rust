use std::ptr;

pub struct List<T> {
    head: RawLink<T>,
    tail: RawLink<T>,
}

type RawLink<T> = *mut Node<T>;

struct Node<T> {
    element: T,
    next: RawLink<T>,
}

impl<T> Node<T> {
    fn new(a: &T) -> RawLink{
        
impl<T> List<T> {
    
