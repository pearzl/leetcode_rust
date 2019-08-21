// q0171_excel_sheet_column_number

struct Solution;

impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        let mut base = 1;
        let map = |c| c as u8 - b'A' + 1;
        let mut ret = 0;
        for c in s.chars().rev() {
            ret += base * (map(c)) as i32;
            base *= 26;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(1, Solution::title_to_number(String::from("A")));
        assert_eq!(28, Solution::title_to_number(String::from("AB")));
        assert_eq!(27, Solution::title_to_number(String::from("AA")));
        assert_eq!(27, Solution::title_to_number(String::from("AA")));
        assert_eq!(26, Solution::title_to_number(String::from("Z")));
        assert_eq!(701, Solution::title_to_number(String::from("ZY")));
        assert_eq!(703, Solution::title_to_number(String::from("AAA")));
        assert_eq!(18278, Solution::title_to_number(String::from("ZZZ")));
        assert_eq!(1352, Solution::title_to_number(String::from("AYZ")));
        assert_eq!(53, Solution::title_to_number(String::from("BA")));
        assert_eq!(78, Solution::title_to_number(String::from("BZ")));
    }
}
