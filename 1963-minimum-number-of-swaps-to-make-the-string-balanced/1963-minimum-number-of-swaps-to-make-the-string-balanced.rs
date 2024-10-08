impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut balance = 0;
        let mut unmatched_closing = 0;
        
        for c in s.chars() {
            if c == '[' {
                balance += 1;
            } else {
                if balance > 0 {
                    balance -= 1;
                } else {
                    unmatched_closing += 1;
                }
            }
        }
        (unmatched_closing + 1) / 2
    }
}