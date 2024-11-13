impl Solution {
    pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut nums = nums;
        nums.sort();
        let mut ans = 0;

        for i in 0..nums.len() {
            let x = nums[i];
            let lower_bound = nums[(i + 1)..].partition_point(|&y| y < lower - x) + i + 1;
            let upper_bound = nums[(i + 1)..].partition_point(|&y| y <= upper - x) + i + 1;
            ans += (upper_bound - lower_bound) as i64;
        }

        ans
    }
}
