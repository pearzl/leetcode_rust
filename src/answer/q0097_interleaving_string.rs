// q0097_interleaving_string

struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let mut check_list: VecDeque<(&str, &str, &str)> = VecDeque::new();
        check_list.push_back((&s1, &s2, &s3));
        loop {
            if let Some((ss1, ss2, ss3)) = check_list.pop_front() {
                if ss1 == "" {
                    if ss2 == ss3 {
                        return true;
                    }
                } else if ss2 == "" {
                    if ss1 == ss3 {
                        return true;
                    }
                } else {
                    let c1 = &ss1[0..=0];
                    let c2 = &ss2[0..=0];
                    let c3 = &ss3[0..=0];
                    if c3 == c2 {
                        let new_combine: (&str, &str, &str) = (&ss1, &ss2[1..], &ss3[1..]);
                        if !check_list.contains(&new_combine) {
                            check_list.push_back(new_combine);
                        }
                    }
                    if c3 == c1 {
                        let new_combine: (&str, &str, &str) = (&ss1[1..], &ss2, &ss3[1..]);
                        if !check_list.contains(&new_combine) {
                            check_list.push_back(new_combine);
                        }
                    }
                }
            } else {
                break;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            true,
            Solution::is_interleave(
                String::from("aabcc"),
                String::from("dbbca"),
                String::from("aadbbcbcac")
            )
        );
        assert_eq!(
            false,
            Solution::is_interleave(
                String::from("aabcc"),
                String::from("dbbca"),
                String::from("aadbbbaccc")
            )
        );
        assert_eq!(
            true,
            Solution::is_interleave(
                String::from("aabc"),
                String::from("abad"),
                String::from("aabcabad")
            )
        );
        assert_eq!( true, Solution::is_interleave(String::from("cbcccbabbccbbcccbbbcabbbabcababbbbbbaccaccbabbaacbaabbbc"), String::from("abcbbcaababccacbaaaccbabaabbaaabcbababbcccbbabbbcbbb"), String::from("abcbcccbacbbbbccbcbcacacbbbbacabbbabbcacbcaabcbaaacbcbbbabbbaacacbbaaaabccbcbaabbbaaabbcccbcbabababbbcbbbcbb")));
    }
}
