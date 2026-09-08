impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let k = k as usize;

        // Caso de fronteira imediato O(1)
        if k >= n {
            return n as i32;
        }

        // 104 bytes na Stack (Cache L1)
        let mut freq = [0u32; 26];
        let mut max_freq: u32 = 0;
        let mut left: usize = 0;
        let ptr = bytes.as_ptr();

        for right in 0..n {
            unsafe {
                let r_char = *ptr.add(right) - b'A';
                let count = freq.get_unchecked_mut(r_char as usize);
                *count += 1;

                if *count > max_freq {
                    max_freq = *count;
                }

                if (right - left + 1) > (max_freq as usize) + k {
                    let l_char = *ptr.add(left) - b'A';
                    *freq.get_unchecked_mut(l_char as usize) -= 1;
                    left += 1;
                }
            }
        }

        (n - left) as i32
    }
}
