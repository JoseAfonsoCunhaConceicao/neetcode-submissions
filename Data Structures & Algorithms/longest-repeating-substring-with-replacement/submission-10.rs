impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let bytes = s.into_bytes();
        let n = bytes.len() as u32;
        let k = k as u32;

        if k >= n {
            return n as i32;
        }

        let mut freq = [0u32; 26];
        let mut max_freq = 0u32;
        let mut left = 0u32;

        unsafe {
            let ptr = bytes.as_ptr();

            for right in 0..n {
                // Leitura direta de 1 byte na memória
                let r_char = (*ptr.add(right as usize) - b'A') as usize;
                let count = freq.get_unchecked_mut(r_char);
                *count += 1;
                
                if *count > max_freq {
                    max_freq = *count;
                }

                // Condição simplificada: right - left + 1 > max_freq + k
                if (right + 1) - left > max_freq + k {
                    let l_char = (*ptr.add(left as usize) - b'A') as usize;
                    *freq.get_unchecked_mut(l_char) -= 1;
                    left += 1;
                }
            }
        }

        (n - left) as i32
    }
}
