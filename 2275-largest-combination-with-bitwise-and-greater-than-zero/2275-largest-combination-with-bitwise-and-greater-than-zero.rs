impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..32 {
            ans = ans.max(candidates.iter().fold(0, |c, &x| c + ((x >> i) & 1) as i32));
        }
        ans
    }
}