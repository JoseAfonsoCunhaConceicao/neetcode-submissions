impl Solution {
    #[inline(always)]
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let k = k as usize;

        if k >= n {
            return n as i32;
        }

        let mut freq = [0u32; 26];
        let mut left: usize = 0;
        let mut max_freq: u32 = 0;
        let ptr = bytes.as_ptr();

        for right in 0..n {
            unsafe {
                // Acesso direto por ponteiro (zero overhead de índice)
                let r_idx = (*ptr.add(right) - b'A') as usize;
                let count = freq.get_unchecked_mut(r_idx);
                *count += 1;
                
                // Hardware conditional move (sem branch misprediction)
                max_freq = max_freq.max(*count);

                // Janela não-decrescente
                if (right - left + 1) - (max_freq as usize) > k {
                    let l_idx = (*ptr.add(left) - b'A') as usize;
                    *freq.get_unchecked_mut(l_idx) -= 1;
                    left += 1;
                }
            }
        }

        (n - left) as i32
    }
}
