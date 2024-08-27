use std::collections::VecDeque;

impl Solution {
    pub fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start_node: i32, end_node: i32) -> f64 {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for i in 0..edges.len() {
            let a = edges[i][0] as usize;
            let b = edges[i][1] as usize;
            let s = succ_prob[i];
            g[a].push((b, s));
            g[b].push((a, s));
        }
        
        let mut d = vec![0.0; n];
        let mut vis = vec![false; n];
        d[start_node as usize] = 1.0;
        let mut q = VecDeque::new();
        q.push_back(start_node as usize);
        vis[start_node as usize] = true;
        
        while let Some(i) = q.pop_front() {
            vis[i] = false;
            for &(j, s) in &g[i] {
                if d[j] < d[i] * s {
                    d[j] = d[i] * s;
                    if !vis[j] {
                        q.push_back(j);
                        vis[j] = true;
                    }
                }
            }
        }
        
        d[end_node as usize]
    }
}