// q0205_isomorphic_strings

struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut map = HashMap::with_capacity(26);
        let mut set = HashSet::with_capacity(26);
        for (sc, tc) in s.chars().zip(t.chars()) {
            if let Some(fc) = map.get(&sc) {
                if *fc != tc {
                    return false;
                }
            } else {
                if set.contains(&tc) {
                    return false;
                } else {
                    map.insert(sc, tc);
                    set.insert(tc);
                }
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            true,
            Solution::is_isomorphic(String::from("egg"), String::from("add"))
        );
        assert_eq!(
            false,
            Solution::is_isomorphic(String::from("foo"), String::from("bar"))
        );
        assert_eq!(
            true,
            Solution::is_isomorphic(String::from("paper"), String::from("title"))
        );
        assert_eq!(
            true,
            Solution::is_isomorphic(String::from(""), String::from(""))
        );
        assert_eq!(
            false,
            Solution::is_isomorphic(String::from("a"), String::from(""))
        );
        assert_eq!(
            false,
            Solution::is_isomorphic(String::from("aa"), String::from("ab"))
        );
        assert_eq!(
            false,
            Solution::is_isomorphic(String::from("ab"), String::from("aa"))
        );
    }
}
