mod list;
mod parse;
mod queue;
mod stack;
mod vec;
use parse::parse_u32;
use stack::Stack;
use vec::Vec;

use crate::list::List;

fn main() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    if let Some(peek) = stack.peek() {
        println!("peek: {}", peek);
    }
    let top = stack.pop().expect("empty");
    println!("Hello, world! {:?} {} {}", stack, stack.size(), top);

    let base16 = parse_u32(15, 16);
    let base2 = parse_u32(16, 2);
    println!("16 to hex is 0x{base16}, to bin 0b{base2}");

    let mut queue = queue::Queue::new(2);
    queue.add(1).unwrap();
    println!(
        "queue size: {}, is empty: {}",
        queue.size(),
        queue.is_empty()
    );

    let mut list = List::new();
    list.push(1);
    list.push(2);
    if let Some(peek) = list.peek() {
        println!("list peek: {}", peek);
    }

    for e in list.iter_mut() {
        *e = 1;
    }

    for v in list.iter() {
        println!("list: {}", v);
    }

    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    println!("vec: {:?}", v);
}
