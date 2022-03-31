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

impl<T: Copy + Debug> Vec<T> {
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
            let mut cur = self.head.as_mut();
            while let Some(v) = cur {
                if v.next.is_none() {
                    v.next = Some(Box::new(node));
                    break;
                }
                cur = v.next.as_mut();
            }
        }
        self.size += 1;
    }

    pub fn insert(&mut self, mut index: usize, data: T) {
        if index > self.size {
            index = self.size
        }
        let mut node = Box::new(Node::new(data));

        if self.is_empty() {
            self.head = Some(node);
        } else {
            let mut cur = self.head.as_mut().unwrap();
            for _i in 0..index - 1 {
                cur = cur.next.as_mut().unwrap();
            }
            node.next = cur.next.take();
            cur.next = Some(node);
        }
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.remove(self.size - 1)
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    // pub fn find(&self, data: T) -> bool {}

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size {
            return None;
        }

        let node;
        if index == 0 {
            node = self.head.take().unwrap();
            self.head = node.next;
        } else {
            let mut cur = self.head.as_mut().unwrap();
            for _i in 0..index - 1 {
                cur = cur.next.as_mut().unwrap();
            }
            node = cur.next.take().unwrap();
            cur.next = node.next;
        }
        self.size -= 1;

        Some(node.data)
    }
}

impl<T: ToString+Debug> ToString for Vec<T> {
    fn to_string(&self) -> String {
        let mut result = String::from("");
        let mut cur = self.head.as_ref();
        while let Some(node) = cur {
            result += &node.data.to_string();
            result += ",";
            cur = node.next.as_ref();
        }
        result
    }
}
