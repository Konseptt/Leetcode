impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let max_val = *candidates.iter().max().unwrap();
        let max_bits = 32 - max_val.leading_zeros() as usize;
        (0..max_bits)
            .map(|i| {
                candidates.iter()
                    .filter(|&&x| (x >> i) & 1 == 1)
                    .count() as i32
            })
            .max()
            .unwrap_or(0)
    }
}