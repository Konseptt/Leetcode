impl Solution {
    pub fn generate_key(num1: i32, num2: i32, num3: i32) -> i32 {
        let num1_str = format!("{:04}", num1);
        let num2_str = format!("{:04}", num2);
        let num3_str = format!("{:04}", num3);

        let mut key = String::new();

        for i in 0..4 {
            let min_digit = num1_str.chars().nth(i).unwrap()
                .min(num2_str.chars().nth(i).unwrap())
                .min(num3_str.chars().nth(i).unwrap());
            
            key.push(min_digit);
        }

        key.parse::<i32>().unwrap()
    }
}