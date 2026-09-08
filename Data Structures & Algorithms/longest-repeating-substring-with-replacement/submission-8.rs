impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let k = k as usize;

        // Se o k cobrir a string toda, podemos transformar tudo na mesma letra
        if k >= n {
            return n as i32;
        }

        let mut freq = [0u32; 128]; // Mapeamento direto ASCII na Cache L1
        let mut left = 0;
        let mut max_freq = 0;

        for (right, &byte) in bytes.iter().enumerate() {
            let count = &mut freq[byte as usize];
            *count += 1;
            
            if *count > max_freq {
                max_freq = *count;
            }

            // Janela não-decrescente: desliza 1 posição se ultrapassar k
            if (right - left + 1) - (max_freq as usize) > k {
                freq[bytes[left] as usize] -= 1;
                left += 1;
            }
        }

        (n - left) as i32
    }
}
