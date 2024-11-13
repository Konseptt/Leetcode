impl Solution {
    pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut ans = 0;

        for i in 0..nums.len() {
            let x = nums[i];
            let lower_bound = (i + 1) + nums[i + 1..].partition_point(|&y| y < lower - x);
            let upper_bound = (i + 1) + nums[i + 1..].partition_point(|&y| y <= upper - x);
            ans += (upper_bound - lower_bound) as i64;
        }

        ans
    }
}
