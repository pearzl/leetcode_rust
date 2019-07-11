// q0005_longest_palindromic_substring

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s = s.as_bytes();
        if s.len() <= 0 {
            return "".to_string();
        }
        let mut tmpr = vec![];

        for i in 0..s.len() {
            tmpr.push((i, i));
            if i != 0 && s[i] == s[i - 1] {
                tmpr.push((i - 1, i))
            }
        }

        for i in 0..tmpr.len() {
            loop {
                let (mut pi, mut li) = tmpr[i];
                if pi <= 0 || li >= s.len() - 1 {
                    break;
                }
                pi -= 1;
                li += 1;
                if s[pi] == s[li] {
                    tmpr[i] = (pi, li);
                } else {
                    break;
                }
            }
        }

        let mut max_index = (0, 0);
        for (a, b) in tmpr.iter() {
            if b - a > max_index.1 - max_index.0 {
                max_index = (*a, *b)
            }
        }

        return String::from_utf8_lossy(&s[max_index.0..=max_index.1]).to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::longest_palindrome(String::from("babad")),
            String::from("bab")
        );
    }
}
