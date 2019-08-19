// q0151_reverse_words_in_a_string

struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let r: Vec<&str> = s.split_ascii_whitespace().rev().collect();
        let mut ret = String::with_capacity(s.len());
        for ss in r.into_iter() {
            ret.push_str(ss);
            ret.push(' ');
        }
        ret.pop();
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            String::from("blue is sky the"),
            Solution::reverse_words(String::from("the sky is blue"))
        );
        assert_eq!(
            String::from("world! hello"),
            Solution::reverse_words(String::from("  hello world!  "))
        );
        assert_eq!(
            String::from("example good a"),
            Solution::reverse_words(String::from("a good   example"))
        );
    }
}
