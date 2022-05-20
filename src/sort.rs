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
