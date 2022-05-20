mod list;
mod num2str;
mod parse;
mod queue;
mod search;
mod sort;
mod stack;
mod vec;
use num2str::num2str;
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
    v.remove(2);
    println!("vec: {:?}", v.to_string());

    println!("num2str: 255 to base16 is 0x{}", num2str(255, 16));

    let numbers = [1, 3, 8, 10, 15, 32, 44, 48, 50, 55, 60, 62, 64];
    let result = search::binary_search(50, &numbers);
    println!("50 is in numbers: {}", result);
    let result = search::binary_search_recursion(40, &numbers);
    println!("40 is in numbers: {}", result);
    let numbers = [1, 9, 10, 15, 16, 17, 19, 23, 27, 28, 29, 30, 32, 35];
    let target = 17;
    let found = search::interpolation_search(target, &numbers);
    println!("{target} is in numbers: {found}");

    let numbers = [1, 9, 10, 15, 16, 17, 19, 23, 27, 28, 29, 30, 32, 35];
    let target = 27;
    let found = search::exponential_search(target, &numbers);
    println!("{target} is in numbers: {found}");

    let mut numbers = [54, 32, 99, 18, 75, 31, 43, 56, 21];
    sort::insertion_sort(&mut numbers);
    println!("insertion_sort numbers: {:?}", numbers);

    let mut numbers = [54, 32, 99, 18, 75, 31, 43, 56, 21, 22];
    sort::shell_sort(&mut numbers);
    println!("shell_sort numbers: {:?}", numbers);
}
