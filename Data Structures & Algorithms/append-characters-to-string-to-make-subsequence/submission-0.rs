impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();

        let s_size = s_bytes.len();

        let t_size = t_bytes.len();

        let mut t_ptr = 0;
        let mut s_ptr = 0;

        while (s_ptr < s_size) && (t_ptr < t_size) {
            if t_bytes[t_ptr] == s_bytes[s_ptr] { t_ptr += 1; } // found the letter
            s_ptr += 1; // always advance in the main sequence
        }

        (t_size - t_ptr) as i32
    }
}
