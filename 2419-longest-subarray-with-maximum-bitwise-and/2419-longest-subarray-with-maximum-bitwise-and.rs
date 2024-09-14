impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mx = *nums.iter().max().unwrap();
        nums.iter().fold((0, 0), |(ans, cnt), &v| {
            if v == mx {
                (ans.max(cnt + 1), cnt + 1)
            } else {
                (ans, 0)
            }
        }).0
    }
}