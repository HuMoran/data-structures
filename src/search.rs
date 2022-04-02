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