// q0087_scramble_string

struct Solution;

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let t: Vec<u8> = s2.as_bytes().iter().cloned().rev().collect();
        let s3 = String::from_utf8(t).unwrap();
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

            let p1s1 = &s2[j..j + ns1.len()];
            let p1s1r = Solution::reverse(p1s1);
            let p1r1 = Solution::is_scramble1(ns1, p1s1) || Solution::is_scramble1(ns1, &p1s1r);
            if p1r1 == true {
                let p1s2 = &s2[j + ns1.len()..];
                let p1s2r = Solution::reverse(p1s2);
                let p1r2 = Solution::is_scramble1(ns2, p1s2) || Solution::is_scramble1(ns2, &p1s2r);
                if p1r2 == true {
                    return true;
                }
            }
            let p2s1 = &s2[j..j + ns2.len()];
            let p2s1r = Solution::reverse(p2s1);
            let p2r1 = Solution::is_scramble1(ns2, p2s1) || Solution::is_scramble1(ns2, &p2s1r);
            if p2r1 == true {
                let p2s2 = &s2[j + ns2.len()..];
                let p2s2r = Solution::reverse(p2s2);
                let p2r2 = Solution::is_scramble1(ns1, p2s2) || Solution::is_scramble1(ns1, &p2s2r);
                if p2r2 == true {
                    return true;
                }
            }
        }
        // println!("S2's first char not found in S1");
        return false;
    }

    fn reverse(s: &str) -> String {
        let t: Vec<u8> = s.as_bytes().iter().cloned().rev().collect();
        String::from_utf8(t).unwrap()
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
