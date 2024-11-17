use std::cmp::min;
use std::collections::VecDeque;

impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut prefix_sums = vec![0i64; n + 1];
        for i in 0..n {
            prefix_sums[i + 1] = prefix_sums[i] + nums[i] as i64;
        }

        let mut deque: VecDeque<usize> = VecDeque::new();
        let mut min_length = n + 1;

        for i in 0..=n {
            while let Some(&front) = deque.front() {
                if prefix_sums[i] - prefix_sums[front] >= k as i64 {
                    min_length = min(min_length, i - front);
                    deque.pop_front();
                } else {
                    break;
                }
            }

            while let Some(&back) = deque.back() {
                if prefix_sums[back] >= prefix_sums[i] {
                    deque.pop_back();
                } else {
                    break;
                }
            }

            deque.push_back(i);
        }

        if min_length > n {
            -1
        } else {
            min_length as i32
        }
    }
}
