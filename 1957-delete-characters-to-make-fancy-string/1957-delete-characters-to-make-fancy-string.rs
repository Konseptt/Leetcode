impl Solution {
    pub fn make_fancy_string(mut s: String) -> String {
        unsafe {
            let bytes = s.as_bytes_mut();
            let mut write = 0;
            for read in 0..bytes.len() {
                if write >= 2 && bytes[read] == bytes[write - 1] && bytes[read] == bytes[write - 2] {
                    continue;
                }
                bytes[write] = bytes[read];
                write += 1;
            }
            s.truncate(write);
        }
        s
    }
}