use std::collections::HashMap;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let mut k = 0;
        for &x in &nums {
            k = (k + x) % p;
        }
        if k == 0 {
            return 0;
        }
        
        let mut last = HashMap::new();
        last.insert(0, -1);
        let n = nums.len() as i32;
        let mut ans = n;
        let mut cur = 0;
        
        for (i, &x) in nums.iter().enumerate() {
            cur = (cur + x) % p;
            let target = (cur - k + p) % p;
            if let Some(&j) = last.get(&target) {
                ans = ans.min(i as i32 - j);
            }
            last.insert(cur, i as i32);
        }
        
        if ans == n {
            -1
        } else {
            ans
        }
    }
}