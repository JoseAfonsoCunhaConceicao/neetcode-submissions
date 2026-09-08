impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();
        let k = k as usize;
        let mut freq = [0usize; 26];
        let mut max_freq = 0;
        let mut left = 0;

        for right in 0..bytes.len() {
            let r_idx = (bytes[right] - b'A') as usize;
            freq[r_idx] += 1;
            max_freq = max_freq.max(freq[r_idx]);

            if right + 1 > left + max_freq + k {
                freq[(bytes[left] - b'A') as usize] -= 1;
                left += 1;
            }
        }

        (bytes.len() - left) as i32
    }
}
