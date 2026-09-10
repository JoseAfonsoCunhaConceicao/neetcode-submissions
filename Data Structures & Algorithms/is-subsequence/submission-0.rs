impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();

        let s_size = s_bytes.len();
        if s_size == 0 { return true; }

        let t_size = t_bytes.len();
        if s_size > t_size { return false; }

        let mut t_ptr = 0;
        let mut s_ptr = 0;

        while (s_ptr < s_size) && (t_ptr < t_size) {
            if s_bytes[s_ptr] == t_bytes[t_ptr] { s_ptr += 1; } // found the letter
            t_ptr += 1; // always advance in the main sequence
        }

        s_ptr == s_size
        
    }
}
