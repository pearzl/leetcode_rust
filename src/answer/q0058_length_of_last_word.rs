// q0058_length_of_last_word

struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut words: Vec<&str> = s.split_ascii_whitespace().collect();
        match words.pop() {
            Some(s) => s.len() as i32,
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::length_of_last_word(String::from("Hello World")),
            5
        );
        assert_eq!(Solution::length_of_last_word(String::from("world")), 5);
        assert_eq!(Solution::length_of_last_word(String::from("   world")), 5);
        assert_eq!(Solution::length_of_last_word(String::from("world    ")), 5);
        assert_eq!(Solution::length_of_last_word(String::from("      ")), 0);
        assert_eq!(Solution::length_of_last_word(String::from("")), 0);
    }
}
