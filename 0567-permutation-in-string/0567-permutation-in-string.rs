use std::collections::HashMap;

impl Solution {
    fn is_match(m1: &HashMap<char, i32>, m2: &HashMap<char, i32>) -> bool {
        for (k, v) in m1.iter() {
            if m2.get(k).unwrap_or(&0) != v {
                return false;
            }
        }
        true
    }
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }
        let mut m1 = HashMap::new();
        let mut m2 = HashMap::new();
        for c in s1.chars() {
            m1.insert(c, m1.get(&c).unwrap_or(&0) + 1);
        }
        let cs: Vec<char> = s2.chars().collect();
        let mut i = 0;
        while i < s1.len() {
            m2.insert(cs[i], m2.get(&cs[i]).unwrap_or(&0) + 1);
            i += 1;
        }
        if Self::is_match(&m1, &m2) {
            return true;
        }
        let mut j = 0;
        while i < cs.len() {
            m2.insert(cs[j], m2.get(&cs[j]).unwrap_or(&1) - 1);
            m2.insert(cs[i], m2.get(&cs[i]).unwrap_or(&0) + 1);
            j += 1;
            i += 1;
            if Self::is_match(&m1, &m2) {
                return true;
            }
        }
        false
    }
}