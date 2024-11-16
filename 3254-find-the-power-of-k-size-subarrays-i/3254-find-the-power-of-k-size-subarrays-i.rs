impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;
        if k == 0 || n < k {
            return Vec::new();
        }
        let mut ans = Vec::with_capacity(n - k + 1);
        let mut streak: u16 = 1;
        for i in 0..n {
            if i > 0 && nums[i] == nums[i - 1] + 1 {
                streak += 1;
            } else {
                streak = 1;
            }
            if i + 1 >= k {
                ans.push(if streak >= k as u16 { nums[i] } else { -1 });
            }
        }
        ans
    }
}
