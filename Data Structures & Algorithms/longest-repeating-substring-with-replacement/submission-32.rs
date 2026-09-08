impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let b = s.as_bytes();
        let n = b.len();
        let k = k as usize;

        if k + 1 >= n {
            return n as i32;
        }

        let mut counts = [0usize; 26];
        let mut max_c = 0;
        let mut l = 0;

        for r in 0..n {
            let r_idx = (b[r] - b'A') as usize;
            counts[r_idx] += 1;
            
            if counts[r_idx] > max_c {
                max_c = counts[r_idx];
            }

            if r + 1 > l + max_c + k {
                counts[(b[l] - b'A') as usize] -= 1;
                l += 1;
            }
        }

        (n - l) as i32
    }
}
