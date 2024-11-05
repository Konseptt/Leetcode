impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let mut ans = 0;
        let bytes = s.as_bytes();
        for i in (1..bytes.len()).step_by(2) {
            if bytes[i] != bytes[i - 1] {
                ans += 1;
            }
        }
        ans
    }
}