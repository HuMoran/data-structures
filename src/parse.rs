use crate::Stack;

pub fn parse_u32(dec_num: u32, base: u32) -> String {
    let mut num = dec_num;
    let digits = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];
    let mut stack = Stack::new();
    while num > 0 {
        stack.push(num % base);
        num /= base;
    }
    let mut result = String::new();

    while !stack.is_empty() {
        let index = stack.pop().unwrap() as usize;
        result += &digits[index].to_string();
    }
    result
}
