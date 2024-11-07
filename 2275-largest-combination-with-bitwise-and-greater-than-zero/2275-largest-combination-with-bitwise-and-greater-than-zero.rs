impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut max_count = 0;
        for i in 0..32 {
            let mut count = 0;
            for &x in candidates.iter() {
                count += ((x >> i) & 1) as i32;
            }
            max_count = max_count.max(count);
        }
        max_count
    }
}