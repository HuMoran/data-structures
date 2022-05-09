pub fn binary_search(target: i32, numbers: &[i32]) -> bool {
    let mut low = 0;
    let mut high = numbers.len() - 1;
    let mut found = false;

    while low <= high && !found {
        let mid = (high - low) / 2 + low;

        if numbers[mid] == target {
            found = true;
        } else if numbers[mid] > target {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    found
}

pub fn binary_search_recursion(target: i32, numbers: &[i32]) -> bool {
    if numbers.len() == 0 {
        return false;
    }

    let mid = numbers.len() >> 1;

    if numbers[mid] == target {
        return true;
    } else if numbers[mid] > target {
        return binary_search_recursion(target, &numbers[..mid]);
    } else {
        return binary_search_recursion(target, &numbers[mid + 1..]);
    }
}

pub fn interpolation_search(target: i32, numbers: &[i32]) -> bool {
    if numbers.is_empty() {
        return false;
    }
    let mut low_index = 0;
    let mut high_index = numbers.len() - 1;
    loop {
        if high_index == low_index {
            return target == numbers[low_index];
        }
        if high_index < low_index || target < numbers[low_index] {
            return false;
        }
        let low = numbers[low_index];
        let high = numbers[high_index];
        let offset = (target - low) / (high - low) * (high_index - low_index) as i32;
        let interpolation_index = low_index + offset as usize;
        if numbers[interpolation_index] > target {
            high_index = interpolation_index - 1;
        } else if numbers[interpolation_index] < target {
            low_index = interpolation_index + 1;
        } else {
            return true;
        }
    }
}
