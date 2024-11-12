use std::cmp::max;

impl Solution {
    pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        items.sort_by(|a, b| a[0].cmp(&b[0]));
        
        let mut idx: Vec<usize> = (0..queries.len()).collect();
        idx.sort_by_key(|&i| queries[i]);

        let mut ans = vec![0; queries.len()];
        let mut i = 0;
        let mut max_beauty = 0;

        for &j in &idx {
            while i < items.len() && items[i][0] <= queries[j] {
                max_beauty = max(max_beauty, items[i][1]);
                i += 1;
            }
            ans[j] = max_beauty;
        }

        ans
    }
}
