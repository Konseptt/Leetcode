impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = arr.len();
        let mut s = vec![0; n + 1];
        for (i, &x) in arr.iter().enumerate() {
            s[i + 1] = s[i] ^ x;
        }
        let mut ans = Vec::new();
        for q in queries {
            let l = q[0] as usize;
            let r = q[1] as usize;
            ans.push(s[r + 1] ^ s[l]);
        }
        ans
    }
}