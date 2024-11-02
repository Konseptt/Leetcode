impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut s = s.into_bytes();
        let mut write = 0;
        for read in 0..s.len() {
            if write >= 2 && s[read] == s[write - 1] && s[read] == s[write - 2] {
                continue;
            }
            s[write] = s[read];
            write += 1;
        }
        s.truncate(write);
        unsafe { String::from_utf8_unchecked(s) }
    }
}