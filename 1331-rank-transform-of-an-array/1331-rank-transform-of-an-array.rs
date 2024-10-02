impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut t = arr.clone();
        t.sort();
        t.dedup();
        
        arr.iter().map(|&x| {
            t.binary_search(&x).unwrap() as i32 + 1
        }).collect()
    }
}