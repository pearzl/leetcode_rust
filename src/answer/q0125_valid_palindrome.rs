// q0125_valid_palindrome

struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: Vec<char> = s
            .chars()
            .filter(char::is_ascii_alphanumeric)
            .map(|c| {
                if c.is_uppercase() {
                    c.to_ascii_lowercase()
                } else {
                    c
                }
            })
            .collect();
        if s.len() == 0 {
            return true;
        }
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            if s[i] == s[j] {
                i += 1;
                j -= 1;
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            true,
            Solution::is_palindrome(String::from("A man, a plan, a canal: Panama"))
        );
        assert_eq!(false, Solution::is_palindrome(String::from("race a car")));
    }
}
