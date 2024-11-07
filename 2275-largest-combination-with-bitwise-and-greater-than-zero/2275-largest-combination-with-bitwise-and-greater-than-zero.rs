impl Solution {
    pub fn largest_combination(v: Vec<i32>) -> i32 {
        (0..32).fold(0, |a, i| a.max(v.iter().fold(0, |c, &x| c + (x >> i & 1) as i32)))
    }
}