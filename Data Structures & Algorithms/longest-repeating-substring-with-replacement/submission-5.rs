impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let k = k as usize;
        
        let mut freq = [0usize; 26];
        let mut left: usize = 0;
        let mut max_freq: usize = 0;
        
        for right in 0..n {
            let idx = (bytes[right] - b'A') as usize;
            freq[idx] += 1;
            
            if freq[idx] > max_freq {
                max_freq = freq[idx];
            }
            let window_size = right - left + 1;
            let replacements_needed = window_size - max_freq;
            
            // Invalid window
            if replacements_needed > k {
                let left_idx = (bytes[left] - b'A') as usize;
                freq[left_idx] -= 1;
                left += 1;
            }
        }
        
        (n - left) as i32
    }
}
