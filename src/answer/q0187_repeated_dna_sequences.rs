// q0187_repeated_dna_sequences

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let l = s.len();
        if l <= 10 {
            return vec![];
        }
        let mut map = HashMap::new();
        for i in 0..=l - 10 {
            let c = map.entry(&s[i..i + 10]).or_insert(0);
            *c += 1;
            println!("{}--{:?}", &s[i..i + 1], map);
        }
        let mut ret = vec![];
        for (k, v) in map.into_iter() {
            if v > 1 {
                ret.push(format!("{}", k));
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util;

    #[test]
    fn it_works() {
        // assert_eq!(
        //     util::vec_2_set(vec![String::from("AAAAACCCCC"), String::from("CCCCCAAAAA")]),
        //     util::vec_2_set(Solution::find_repeated_dna_sequences(String::from(
        //         "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"
        //     )))
        // );
        assert_eq!(
            util::vec_2_set(vec![String::from("AAAAAAAAAA")]),
            util::vec_2_set(Solution::find_repeated_dna_sequences(String::from(
                "AAAAAAAAAAA"
            )))
        );
    }
}
