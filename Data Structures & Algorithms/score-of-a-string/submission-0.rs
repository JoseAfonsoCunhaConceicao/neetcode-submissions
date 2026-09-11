impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let s_bytes = s.as_bytes();
        let s_size = s_bytes.len();

        let mut some: i32 = 0;

        for letter in (0..(s_size - 1)) {
            some += (s_bytes[letter + 1] as i32 - s_bytes[letter] as i32).abs();
        }

        some
    }
}
