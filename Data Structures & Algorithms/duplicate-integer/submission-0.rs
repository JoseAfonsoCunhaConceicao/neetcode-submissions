use std::collections::HashSet;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let size: usize = nums.len();

        if (size <= 1) {return false;} /*the existence of duplicates is impossible*/

        let mut set: HashSet<i32> = HashSet::with_capacity(size);

        for num in nums {
            if !set.insert(num) {
                return true;
            }
        }

        false
    }
}