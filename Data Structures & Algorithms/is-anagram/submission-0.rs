impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {return false;}

        let mut letters = [0; 26];
        for letter in s.bytes() {
            letters[(letter - b'a') as usize] += 1;
        }
        
        for letter in t.bytes() {
            let idx = (letter - b'a') as usize;
            letters[idx] -= 1;
            if letters[idx] < 0 { return false; }
        }
        true
    }
}
