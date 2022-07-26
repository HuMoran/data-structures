use core::num;

pub fn insertion_sort(numbers: &mut [i32]) {
  for i in 1..numbers.len() {
      let mut pos = i;
      let cur = numbers[pos];
      while pos > 0 && cur < numbers[pos - 1] {
          numbers[pos] = numbers[pos - 1];
          pos -= 1;
      }
      numbers[pos] = cur;
  }
}

pub fn shell_sort(numbers: &mut [i32]) {
    fn ist_sort(numbers: &mut [i32], start: usize, gap: usize) {
        let mut i = start + gap;
        while i < numbers.len() {
            let mut pos = i;
            let cur = numbers[pos];
            while pos >= gap && cur < numbers[pos - gap] {
                numbers[pos] = numbers[pos - gap];
                pos -= gap;
            }
            numbers[pos] = cur;
            i += gap;
        }
    }
    let mut gap = numbers.len() / 2;
    while gap > 0 {
        for start in 0..gap {
            ist_sort(numbers, start, gap);
        }
        gap /= 2;
    }
}

pub fn merge_sort(numbers: &mut [i32]) {
    if numbers.len() > 1 {
        let mid = numbers.len() >> 1;
        merge_sort(&mut numbers[..mid]);
        merge_sort(&mut numbers[mid..]);
        merge(numbers, mid);
    }
}

fn merge(numbers: &mut [i32], mid: usize) {
    let mut i = 0;
    let mut k = mid;
    let mut tmp_numbers = vec![];

    for _ in 0..numbers.len() {
        if i == mid || k == numbers.len() {
            break;
        }
        if numbers[i] < numbers[k] {
            tmp_numbers.push(numbers[i]);
            i += 1;
        } else {
            tmp_numbers.push(numbers[k]);
            k += 1;
        }
    }

    if i < mid && k == numbers.len() {
        for j in i..mid {
            tmp_numbers.push(numbers[j]);
        }
    }
    if k < numbers.len() && i == mid {
        for j in k..numbers.len() {
            tmp_numbers.push(numbers[j]);
        }
    }
    for j in 0..numbers.len() {
        numbers[j] = tmp_numbers[j];
    }
}