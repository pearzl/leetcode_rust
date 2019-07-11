// q0009_palindrome_number

struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let s = x.to_string();
        let sb = s.as_bytes();
        let mut i = 0;
        let mut j = sb.len() - 1;
        loop {
            if i >= j {
                break;
            }
            if sb[i] != sb[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_palindrome(121), true);
    }
}
