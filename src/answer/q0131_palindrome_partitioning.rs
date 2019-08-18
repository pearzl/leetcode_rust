// q0131_palindrome_partitioning

struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let ss = s.as_str();
        let mut ret = vec![];
        for i in 0..ss.len() {
            if Solution::is_plalindrome(&ss[0..=i]) {
                println!("{}-{}", i, &ss[0..=i]);
                if i == ss.len() - 1 {
                    ret.push(vec![format!("{}", &ss[0..=i])]);
                    continue;
                }
                let vvs = Solution::partition(format!("{}", &ss[i + 1..]));
                for mut vs in vvs.into_iter() {
                    vs.insert(0, format!("{}", &ss[0..=i]));
                    ret.push(vs);
                }
            }
        }
        ret
    }

    fn is_plalindrome(s: &str) -> bool {
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            if s[i..=i] != s[j..=j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    use crate::util;

    fn string_array(v: Vec<&str>) -> Vec<String> {
        v.into_iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn it_works() {
        assert_eq!(
            util::vec_2_set(vec![
                string_array(vec!["aa", "b"]),
                string_array(vec!["a", "a", "b"])
            ]),
            util::vec_2_set(Solution::partition(String::from("aab")))
        );
    }
}
