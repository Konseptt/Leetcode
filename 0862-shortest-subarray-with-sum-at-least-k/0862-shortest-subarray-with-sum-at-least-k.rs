use std::cmp::min;


impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as i64;
        let mut prefix_sums = Vec::with_capacity(n + 1);
        prefix_sums.push(0i64);
        for &num in &nums {
            prefix_sums.push(prefix_sums[prefix_sums.len() - 1] + num as i64);
        }

        let mut deque = Vec::with_capacity(n + 1);
        let mut head = 0;
        let mut min_length = n + 1;

        for i in 0..=n {
            while head < deque.len() && prefix_sums[i] - prefix_sums[deque[head]] >= k {
                min_length = min(min_length, i - deque[head]);
                head += 1;
            }

            while deque.len() > head && prefix_sums[i] <= prefix_sums[*deque.last().unwrap()] {
                deque.pop();
            }

            deque.push(i);
        }

        if min_length <= n {
            min_length as i32
        } else {
            -1
        }
    }
}
