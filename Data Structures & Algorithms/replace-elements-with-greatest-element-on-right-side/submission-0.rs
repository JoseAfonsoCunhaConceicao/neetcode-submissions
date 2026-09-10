use std::cmp::max;

impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        let size = arr.len();
        let mut max_value = -1; // First element replacement is the last in the array

        for idx in (0..size).rev() {
            let current_value = arr[idx];
            arr[idx] = max_value;
            max_value = max(max_value, current_value);
        }

        return arr;
    }
}
