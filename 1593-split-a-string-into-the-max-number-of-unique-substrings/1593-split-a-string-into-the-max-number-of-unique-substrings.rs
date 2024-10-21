use std::collections::HashSet;

impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        let mut ans = 1;
        let mut vis = HashSet::new();
        
        fn dfs(s: &str, i: usize, t: i32, vis: &mut HashSet<String>, ans: &mut i32) {
            if i >= s.len() {
                *ans = (*ans).max(t);
                return;
            }
            for j in i + 1..=s.len() {
                let x = s[i..j].to_string();
                if !vis.contains(&x) {
                    vis.insert(x.clone());
                    dfs(s, j, t + 1, vis, ans);
                    vis.remove(&x);
                }
            }
        }

        dfs(&s, 0, 0, &mut vis, &mut ans);
        ans
    }
}
