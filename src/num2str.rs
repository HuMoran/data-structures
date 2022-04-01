const BASE_STR: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];

pub fn num2str(num: i32, base: i32) -> String {
    if num < base {
      BASE_STR[num as usize].to_string()
    } else {
      num2str(num/base, base) + &BASE_STR[(num%base) as usize].to_string()
    }
}