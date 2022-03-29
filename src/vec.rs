use std::fmt::Debug;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node { data, next: None }
    }
}

#[derive(Debug)]
pub struct Vec<T> {
    size: usize,
    head: Link<T>,
}

impl<T: Debug> Vec<T> {
    pub fn new() -> Self {
        Vec {
            size: 0,
            head: None,
        }
    }

    pub fn push(&mut self, data: T) {
        let node = Node::new(data);
        if self.is_empty() {
            self.head = Some(Box::new(node));
        } else {
            let mut n = &mut self.head;
            while let Some(v) = n {
                println!("vec data:{:?} {:?}", v.data, v.next.is_none());
                if v.next.is_none() {
                    v.next = Some(Box::new(node));
                    break;
                }
                n = &mut v.next;
            }
        }
        self.size += 1;
    }

    // pub fn pop(&self) -> Option<T> {}

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }

    // pub fn find(&self, data: T) -> bool {}

    // pub fn insert() {}

    // pub fn remove() -> Option<T> {}
}
