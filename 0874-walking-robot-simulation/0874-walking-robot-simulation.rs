use std::collections::HashSet;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let dirs = [0, 1, 0, -1, 0];
        let mut s = HashSet::new();
        for e in obstacles {
            s.insert((e[0], e[1]));
        }
        let (mut x, mut y, mut k) = (0, 0, 0);
        let mut ans = 0;
        for c in commands {
            if c == -2 {
                k = (k + 3) % 4;
            } else if c == -1 {
                k = (k + 1) % 4;
            } else {
                for _ in 0..c {
                    let nx = x + dirs[k];
                    let ny = y + dirs[k + 1];
                    if s.contains(&(nx, ny)) {
                        break;
                    }
                    x = nx;
                    y = ny;
                    ans = ans.max(x * x + y * y);
                }
            }
        }
        ans
    }
}