use std::collections::BinaryHeap;

impl Solution {
    pub fn results_array(queries: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut results = Vec::new();
        let mut max_heap = BinaryHeap::new();
        
        for query in queries {
            let x = query[0];
            let y = query[1];
            let distance = x.abs() + y.abs();
            
            if max_heap.len() < k as usize {
                max_heap.push(distance);
            } else if *max_heap.peek().unwrap() > distance {
                max_heap.pop();
                max_heap.push(distance);
            }
            
            if max_heap.len() < k as usize {
                results.push(-1);
            } else {
                results.push(*max_heap.peek().unwrap());
            }
        }
        
        results
    }
}