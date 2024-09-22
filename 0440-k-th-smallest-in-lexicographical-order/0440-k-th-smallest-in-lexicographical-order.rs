impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        fn count(curr: i64, n: i64) -> i64 {
            let mut next = curr + 1;
            let mut cnt = 0;
            let mut curr = curr;
            while curr <= n {
                cnt += std::cmp::min(n - curr + 1, next - curr);
                next *= 10;
                curr *= 10;
            }
            cnt
        }

        let mut curr = 1i64;
        let mut k = k as i64 - 1;
        let n = n as i64;
        while k > 0 {
            let cnt = count(curr, n);
            if k >= cnt {
                k -= cnt;
                curr += 1;
            } else {
                k -= 1;
                curr *= 10;
            }
        }
        curr as i32
    }
}