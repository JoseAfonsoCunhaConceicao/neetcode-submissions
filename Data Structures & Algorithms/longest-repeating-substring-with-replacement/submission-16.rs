impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let k = k as usize;

        // 1. Caso de fronteira imediato O(1)
        if k + 1 >= n {
            return n as i32;
        }

        let mut count = [0usize; 26];
        let mut max_count = 0;
        let mut left = 0;

        // 2. Iterador direto (elimina cálculo de índices de leitura)
        for (right, &b) in bytes.iter().enumerate() {
            let idx = (b - b'A') as usize;
            count[idx] += 1;
            
            if count[idx] > max_count {
                max_count = count[idx];
            }

            // 3. Verificação ultra-rápida só com adições
            if right + 1 > left + max_count + k {
                count[(bytes[left] - b'A') as usize] -= 1;
                left += 1;
            }
        }

        (n - left) as i32
    }
}
