use std::cmp::max;

impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let mut mask = 0;
        let mut first_seen: [Option<usize>; 32] = [None; 32];
        first_seen[0] = Some(0);
        let mut max_len = 0;
        for (i, c) in s.chars().enumerate() {
            mask ^= match c {
                'a' => 1 << 0,
                'e' => 1 << 1,
                'i' => 1 << 2,
                'o' => 1 << 3,
                'u' => 1 << 4,
                _ => 0,
            };
            if let Some(prev_index) = first_seen[mask as usize] {
                let current_len = i + 1 - prev_index;
                max_len = max(max_len, current_len);
            } else {
                first_seen[mask as usize] = Some(i + 1);
            }
        }
        max_len as i32
    }
}
