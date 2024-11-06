use std::cmp::{min, max};

impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        let mut pre_max = 0;
        let mut i = 0;
        let n = nums.len();
        
        while i < n {
            let cnt = nums[i].count_ones();
            let mut j = i + 1;
            let mut mi = nums[i];
            let mut mx = nums[i];
            
            while j < n && nums[j].count_ones() == cnt {
                mi = min(mi, nums[j]);
                mx = max(mx, nums[j]);
                j += 1;
            }
            
            if pre_max > mi {
                return false;
            }
            
            pre_max = mx;
            i = j;
        }
        
        true
    }
}
