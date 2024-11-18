impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let n = code.len();
        let mut ans = vec![0; n];

        if k == 0 {
            return ans;
        }

        for i in 0..n {
            if k > 0 {
                for j in i + 1..=i + k as usize {
                    ans[i] += code[j % n];
                }
            } else {
                for j in (i as i32 + k)..i as i32 {
                    ans[i] += code[((j + n as i32) % n as i32) as usize];
                }
            }
        }

        ans
    }
}
