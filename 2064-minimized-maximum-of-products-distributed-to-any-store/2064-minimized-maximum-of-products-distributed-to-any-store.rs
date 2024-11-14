impl Solution {
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let n = n as i64;
        let max_q = *quantities.iter().max().unwrap_or(&1);
        let mut left = 1;
        let mut right = max_q;
        
        while left < right {
            let mid = left + (right - left) / 2;
            let mut cnt = 0;
            for &q in &quantities {
                cnt += ((q + mid - 1) / mid) as i64;
                if cnt > n {
                    break;
                }
            }
            if cnt <= n {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}
