impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let mut ans = 0;
        for i in (1..s.len()).step_by(2) {
            if s.as_bytes()[i] != s.as_bytes()[i - 1] {
                ans += 1;
            }
        }
        ans
    }
}