// q0139_word_break

struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let l = word_dict.len();
        let mut words_len = HashSet::with_capacity(l);
        let mut words_dict = HashSet::with_capacity(l);
        for s in word_dict.into_iter() {
            words_len.insert(s.len());
            words_dict.insert(s);
        }
        let ss = s.as_str();
        let ssl = ss.len();
        let mut start_pos = HashSet::new();
        start_pos.insert(0);
        // println!("{:?}__{:?}", words_dict, words_len);
        while !start_pos.is_empty() {
            let mut new_start_pos = HashSet::new();
            for i in start_pos.iter().cloned() {
                for len in words_len.iter().cloned() {
                    // println!("{}--{}--{}", ssl, i, len);
                    if ssl - i < len {
                        continue;
                    }
                    let nsp = i + len;
                    // println!("  {:?}", &ss[i..nsp]);
                    if words_dict.contains(&ss[i..nsp]) {
                        if nsp == ssl {
                            return true;
                        }
                        new_start_pos.insert(nsp);
                    }
                }
            }
            println!("{:?}--{:?}", start_pos, new_start_pos);
            std::mem::swap(&mut new_start_pos, &mut start_pos);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    use crate::util;

    #[test]
    fn it_works() {
        assert_eq!(
            true,
            Solution::word_break(
                String::from("applepenapple"),
                util::build_string_array(vec!["apple", "pen"])
            )
        );
        assert_eq!(
            true,
            Solution::word_break(
                String::from("leetcode"),
                util::build_string_array(vec!["leet", "code"])
            )
        );
        assert_eq!(
            false,
            Solution::word_break(
                String::from("catsandog"),
                util::build_string_array(vec!["cats", "dog", "sand", "and", "cat"])
            )
        );
    }
}
