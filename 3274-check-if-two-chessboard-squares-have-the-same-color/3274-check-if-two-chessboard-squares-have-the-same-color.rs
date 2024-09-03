impl Solution {
    pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
        fn is_black(coordinate: &str) -> bool {
            let column = coordinate.chars().next().unwrap() as u8 - b'a' + 1;
            let row = coordinate.chars().nth(1).unwrap().to_digit(10).unwrap();
            (column + row as u8) % 2 == 0
        }

        is_black(&coordinate1) == is_black(&coordinate2)
    }
}