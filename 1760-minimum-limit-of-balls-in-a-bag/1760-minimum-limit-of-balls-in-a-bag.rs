impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let max_value = *nums.iter().max().unwrap();
        
        let mut left = 1;
        let mut right = max_value;
        
        while left < right {
            let mid = (left + right) / 2;
            if Self::can_split(&nums, mid, max_operations) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        
        left
    }

    fn can_split(nums: &Vec<i32>, max_size: i32, max_operations: i32) -> bool {
        let mut operations = 0;
        for &num in nums.iter() {
            operations += (num - 1) / max_size;
            if operations > max_operations {
                return false;
            }
        }
        true
    }
}
