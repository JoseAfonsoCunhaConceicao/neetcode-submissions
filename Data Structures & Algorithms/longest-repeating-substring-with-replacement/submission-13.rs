impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut count = [0i32; 26];
        let mut max_count = 0;
        let mut left = 0;
        let bytes = s.as_bytes();
        let n = bytes.len() as i32;

        for right in 0..n {
            let r_idx = (bytes[right as usize] - b'A') as usize;
            count[r_idx] += 1;
            max_count = max_count.max(count[r_idx]);

            if (right - left + 1) - max_count > k {
                let l_idx = (bytes[left as usize] - b'A') as usize;
                count[l_idx] -= 1;
                left += 1;
            }
        }

        n - left
    }
}
