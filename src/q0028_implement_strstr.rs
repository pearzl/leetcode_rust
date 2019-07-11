// q0028_implement_strstr

struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(needle.as_str()) {
            Some(n) => n as i32,
            None => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(2, Solution::str_str("hello".to_string(), "ll".to_string()));
        assert_eq!(
            -1,
            Solution::str_str("aaaaa".to_string(), "bba".to_string())
        );
    }
}
