impl Solution {
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let mut left = 1;
        let mut right = *quantities.iter().max().unwrap_or(&1);
        while left < right {
            let mid = left + (right - left) / 2;
            let cnt: i32 = quantities.iter().map(|&v| (v + mid - 1) / mid).sum();
            if cnt <= n {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}
