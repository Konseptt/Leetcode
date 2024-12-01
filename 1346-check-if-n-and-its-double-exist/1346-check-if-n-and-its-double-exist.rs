impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut seen = std::collections::HashSet::new();
        
        for &x in &arr {
            if seen.contains(&(x * 2)) || (x % 2 == 0 && seen.contains(&(x / 2))) {
                return true;
            }
            seen.insert(x);
        }
        
        false
    }
}
