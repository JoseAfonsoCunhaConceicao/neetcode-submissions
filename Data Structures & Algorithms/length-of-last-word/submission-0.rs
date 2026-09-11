impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let s_bytes = s.as_bytes();
        let s_size = s_bytes.len();

        let mut last_word_size = 0;
        let mut current_word_size = 0;

        for idx in 0..s_size {
            if let Some(&byte) = s_bytes.get(idx) {
                if (byte == b' ') && (current_word_size != 0) {
                    last_word_size = current_word_size;
                    current_word_size = 0;
                } else if  (byte == b' ') && (current_word_size == 0) {
                    continue;
                } else {
                    current_word_size += 1;
                }
            }
        }

        if current_word_size != 0 { last_word_size = current_word_size; }
        last_word_size
    }

}
