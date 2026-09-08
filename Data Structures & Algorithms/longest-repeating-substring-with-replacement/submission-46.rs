impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let b = s.as_bytes();
        let k = k as usize;

        if k + 1 >= b.len() {
            return b.len() as i32;
        }

        // Compact table with 104 bytes in the stack
        let mut count = [0u32; 26];
        let mut max_c = 0u32;
        let mut left = 0usize;

        for (right, &byte) in b.iter().enumerate() {
            let idx = (byte - b'A') as usize;
            count[idx] += 1;
            
            if count[idx] > max_c {
                max_c = count[idx];
            }

            if right + 1 > left + (max_c as usize) + k {
                count[(b[left] - b'A') as usize] -= 1;
                left += 1;
            }
        }

        (b.len() - left) as i32
    }
}
