use std::collections::HashMap;

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let mut memo = HashMap::new();
        Self::dfs(&expression, &mut memo)
    }

    fn dfs(exp: &str, memo: &mut HashMap<String, Vec<i32>>) -> Vec<i32> {
        if let Some(v) = memo.get(exp) {
            return v.clone();
        }
        if exp.len() < 3 {
            if let Ok(v) = exp.parse::<i32>() {
                return vec![v];
            }
        }
        let mut ans = vec![];
        for (i, c) in exp.chars().enumerate() {
            if c == '-' || c == '+' || c == '*' {
                let left = Self::dfs(&exp[..i], memo);
                let right = Self::dfs(&exp[i+1..], memo);
                for &a in &left {
                    for &b in &right {
                        if c == '-' {
                            ans.push(a - b);
                        } else if c == '+' {
                            ans.push(a + b);
                        } else {
                            ans.push(a * b);
                        }
                    }
                }
            }
        }
        memo.insert(exp.to_string(), ans.clone());
        ans
    }
}