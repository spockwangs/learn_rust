use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use std::marker::PhantomData;

struct Node<T> {
    a: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(a: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            a: a,
            next: None,
        }))
    }
}

struct List<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

struct ListIterator<'a, T: 'a> {
    first: bool,
    cur: Option<Rc<RefCell<Node<T>>>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> ListIterator<'a, T> {
    fn new(l: &List<T>) -> ListIterator<'a, T> {
        ListIterator {
            first: true,
            cur: l.head.clone(),
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for ListIterator<'a, T>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.first {
            let next = self.cur.and_then(|x| x.borrow().next.clone());
            self.cur = next;
            if self.cur.is_none() {
                return None;
            }
        }
        match &self.cur {
            None => None,
            Some(x) => Some(&(x.borrow().a)),
        }
    }
}

impl<T> List<T> {
    fn new() -> List<T> {
        List {
            head: None,
            tail: None,
        }
    }

    fn push(&mut self, a: T) {
        let x = Node::new(a);
        if self.head.is_none() {
            self.head = Some(Rc::clone(&x));
            self.tail = Some(x);
        } else {
            if let Some(n) = &self.tail {
                n.borrow_mut().next = Some(x.clone());
            }
            self.tail = Some(x);
        }
    }

    fn pop(&mut self) -> Option<T>
    {
        match self.head.take() {
            None => None,
            Some(old_head) => {
                self.head = old_head.borrow().next.clone();
                if self.head.is_none() {
                    self.tail = None;
                }
                Some(Rc::try_unwrap(old_head).ok().unwrap().into_inner().a)
            },
        }
    }

    fn front(&self) -> Option<T>
    where T: Clone        
    {
        match &self.head {
            None => None,
            Some(x) => Some(x.borrow().a.clone()),
        }
    }

    fn back(&self) -> Option<T>
    where T: Clone
    {
        match &self.tail {
            None => None,
            Some(x) => Some(x.borrow().a.clone()),
        }
    }

    fn iter(&self) -> ListIterator<T>
    where T: Clone
    {
        ListIterator::new(&self)
    }
}

impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut cur = self.head.clone();
        while let Some(x) = cur {
            write!(f, "{}->", &x.borrow().a)?;
            cur = x.borrow().next.clone();
        }
        write!(f, "()")
    }
}


fn main() {
    let mut l = List::<i32>::new();
    l.push(1);
    l.push(2);
    l.push(3);
    println!("{}", l);
    for it in l.iter() {
        println!("{}", it);
    }
}
