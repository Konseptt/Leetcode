impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut result = String::with_capacity(s.len());
        let mut prev1 = '\0';
        let mut prev2 = '\0';
        
        for c in s.chars() {
            if c != prev1 || c != prev2 {
                result.push(c);
                prev2 = prev1;
                prev1 = c;
            }
        }
        
        result
    }
}