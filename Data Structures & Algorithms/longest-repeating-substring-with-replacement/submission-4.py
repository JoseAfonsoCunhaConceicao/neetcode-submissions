class Solution:
    def characterReplacement(self, s: str, k: int) -> int:
        letters_frequency = [0] * 26
        left = 0
        max_frequency = 0

        #working with bytes in order to not call multiple times the ord function
        s_bytes = s.encode("ascii")

        for right in range(len(s_bytes)):
            idx = s_bytes[right] - 65  # 65 == ord('A')
            letters_frequency[idx] += 1
        
            if letters_frequency[idx] > max_frequency:
                max_frequency = letters_frequency[idx]

            #invalid window
            if (right - left + 1) - max_frequency > k:
                letters_frequency[s_bytes[left] - 65] -= 1
                left += 1
        
        return len(s_bytes) - left