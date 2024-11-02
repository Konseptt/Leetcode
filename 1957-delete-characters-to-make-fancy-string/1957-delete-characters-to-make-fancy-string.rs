impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut ans = Vec::with_capacity(s.len());
        for c in s.chars() {
            let n = ans.len();
            if n < 2 || c != ans[n - 1] || c != ans[n - 2] {
                ans.push(c);
            }
        }
        ans.into_iter().collect()
    }
}