impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut vs: Vec<String> = nums.iter().map(|&num| num.to_string()).collect();
        
        vs.sort_by(|a, b| (b.clone() + a).cmp(&(a.clone() + b)));
        
        if vs[0] == "0" {
            return "0".to_string();
        }
        
        vs.join("")
    }
}