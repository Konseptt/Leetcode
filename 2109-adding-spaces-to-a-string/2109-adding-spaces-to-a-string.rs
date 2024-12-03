impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut ans = String::new();
        let mut j = 0;
        
        for (i, ch) in s.chars().enumerate() {
            if j < spaces.len() && i == spaces[j] as usize {
                ans.push(' ');
                j += 1;
            }
            ans.push(ch);
        }
        
        ans
    }
}