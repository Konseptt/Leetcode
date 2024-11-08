impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut xs = 0;
        for x in &nums {
            xs ^= x;
        }

        let mask = (1 << maximum_bit) - 1;

        let mut ans = Vec::with_capacity(nums.len());
        for i in 0..nums.len() {
            let x = nums[nums.len() - i - 1];
            let k = xs ^ mask;
            ans.push(k);
            xs ^= x;
        }

        ans
    }
}