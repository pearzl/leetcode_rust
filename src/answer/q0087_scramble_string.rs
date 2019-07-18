// q0087_scramble_string

struct Solution;

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let s3: Vec<u8> = s2.as_bytes().iter().cloned().rev().collect();
        let s3 = String::from_utf8(s3).unwrap();
        Solution::is_scramble1(&s1, &s2) || Solution::is_scramble1(&s1, &s3)
    }

    fn is_scramble1(s1: &str, s2: &str) -> bool {
        // println!("handling {}--{}", s1, s2);
        if s1 == s2 {
            // println!("equals");
            return true;
        }
        for (i, _) in s1.match_indices(&s2[0..=0]) {
            let mut j = 1;
            while j + i < s2.len() {
                if s2[j..=j] != s1[j + i..=j + i] {
                    break;
                }
                j += 1;
            }
            let ns1 = &s1[0..i];
            let ns2 = &s1[j + i..];
            // println!("split S1 to {} -- {}", ns1, ns2);
            // println!("split S2 to {} -- {}", &s2[j..j+ns1.len()], &s2[j+ns1.len()..]);
            // println!("split S2 to {} -- {}", &s2[j..j+ns2.len()], &s2[j+ns2.len()..]);
            let r1 = Solution::is_scramble1(ns1, &s2[j..j + ns1.len()])
                && Solution::is_scramble1(ns2, &s2[j + ns1.len()..]);
            if r1 == true {
                return true;
            }
            let r2 = Solution::is_scramble1(ns2, &s2[j..j + ns2.len()])
                && Solution::is_scramble1(ns1, &s2[j + ns2.len()..]);
            if r2 == true {
                return true;
            }
        }
        // println!("S2's first char not found in S1");
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            true,
            Solution::is_scramble(String::from("great"), String::from("rgeat"))
        );
        assert_eq!(
            false,
            Solution::is_scramble(String::from("abcde"), String::from("caebd"))
        );
        assert_eq!(
            true,
            Solution::is_scramble(String::from("asdfewrsdf"), String::from("asfdrsewdf"))
        );
        assert_eq!(
            true,
            Solution::is_scramble(String::from("ab"), String::from("ba"))
        );
        assert_eq!(
            true,
            Solution::is_scramble(String::from("bbaa"), String::from("abab"))
        );
        assert_eq!(
            true,
            Solution::is_scramble(String::from("abcd"), String::from("cbda"))
        );
        assert_eq!(
            true,
            Solution::is_scramble(String::from("abcd"), String::from("adbc"))
        );
        assert_eq!(
            true,
            Solution::is_scramble(String::from("abc"), String::from("acb"))
        );
        assert_eq!(
            true,
            Solution::is_scramble(String::from("abc"), String::from("bac"))
        );
        assert_eq!(
            true,
            Solution::is_scramble(String::from("abc"), String::from("bca"))
        );
        assert_eq!(
            true,
            Solution::is_scramble(String::from("abc"), String::from("cab"))
        );
        assert_eq!(
            true,
            Solution::is_scramble(String::from("abc"), String::from("cba"))
        );
    }
}
