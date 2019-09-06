// q0179_largest_number

struct Solution;
use std::io::Write;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut t1: Vec<Number> = nums.into_iter().map(|x| Number(format!("{}", x))).collect();
        t1.sort_unstable();
        let mut s = Vec::new();
        for n in t1.into_iter().rev() {
            let _ = write!(&mut s, "{}", n.0);
        }
        if s[0] == b'0' {
            String::from("0")
        } else {
            String::from_utf8(s).unwrap()
        }
    }
}

#[derive(PartialEq, Eq)]
struct Number(String);

use std::cmp::Ordering;

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 == other.0 {
            Some(Ordering::Equal)
        } else {
            let gt = format!("{}{}", self.0, other.0);
            let ls = format!("{}{}", other.0, self.0);
            gt.partial_cmp(&ls)
        }
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(String::from("210"), Solution::largest_number(vec![10, 2]));
        assert_eq!(
            String::from("9534330"),
            Solution::largest_number(vec![3, 30, 34, 5, 9])
        );
        assert_eq!(
            String::from("12121"),
            Solution::largest_number(vec![121, 12])
        );
        assert_eq!(
            String::from("9609938824824769735703560743981399"),
            Solution::largest_number(vec![824, 938, 1399, 5607, 6973, 5703, 9609, 4398, 8247])
        );
    }
}
