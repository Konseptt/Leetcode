impl Solution {
    pub fn min_end(mut n: i32, x: i32) -> i64 {
        n -= 1;
        let mut ans = x as i64;
        
        for i in 0..31 {
            if (x >> i) & 1 == 0 {
                ans |= ((n & 1) as i64) << i;
                n >>= 1;
            }
        }
        
        ans |= (n as i64) << 31;
        ans
    }
}
