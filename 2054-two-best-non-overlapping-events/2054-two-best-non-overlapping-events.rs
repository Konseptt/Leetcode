impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        let mut events = events;
        events.sort_by_key(|e| e[0]);
        let n = events.len();
        let mut f = vec![0; n + 1];
        for i in (0..n).rev() {
            f[i] = f[i + 1].max(events[i][2]);
        }
        let mut ans = 0;
        for e in &events {
            let mut left = 0;
            let mut right = n;
            while left < right {
                let mid = (left + right) / 2;
                if events[mid][0] > e[1] {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            let mut v = e[2];
            if left < n {
                v += f[left];
            }
            ans = ans.max(v);
        }
        ans
    }
}
