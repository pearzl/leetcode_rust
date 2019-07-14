// q0049_group_anagrams

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut count = vec![];
        for s in strs.iter() {
            let mut tmp: [usize; 26] = [0; 26]; // 当前字符串在每个字母上出现的次数
            for c in s.as_bytes() {
                tmp[(c - b'a') as usize] += 1;
            }
            count.push(tmp);
        }

        let mut picked = vec![false; strs.len()];
        let mut ret = vec![];
        for i in 0..strs.len() {
            if picked[i] {
                continue;
            }
            let mut tv = vec![];
            tv.push(strs[i].clone());
            picked[i] = true;
            for j in i + 1..strs.len() {
                if count[i] == count[j] {
                    tv.push(strs[j].clone());
                    picked[j] = true;
                }
            }
            ret.push(tv);
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
        assert!(util::compare_nest2_vec(
            Solution::group_anagrams(vec![
                String::from("eat"),
                String::from("tea"),
                String::from("tan"),
                String::from("ate"),
                String::from("nat"),
                String::from("bat")
            ]),
            vec![
                vec![
                    String::from("ate"),
                    String::from("eat"),
                    String::from("tea")
                ],
                vec![String::from("nat"), String::from("tan")],
                vec![String::from("bat")]
            ]
        ));

        assert!(util::compare_nest2_vec(
            Solution::group_anagrams(vec![
                String::from("tea"),
                String::from("and"),
                String::from("ace"),
                String::from("ad"),
                String::from("eat"),
                String::from("dans")
            ]),
            vec![
                vec![String::from("eat"), String::from("tea")],
                vec![String::from("and")],
                vec![String::from("dans")],
                vec![String::from("ace")],
                vec![String::from("ad")]
            ]
        ));
        assert!(util::compare_nest2_vec(
            Solution::group_anagrams(vec![String::from(""), String::from("bat")]),
            vec![vec![String::from("bat")], vec![String::from("")]]
        ));
        assert!(util::compare_nest2_vec(
            Solution::group_anagrams(vec![String::from(""), String::from("ill")]),
            vec![vec![String::from("ill")], vec![String::from("")]]
        ));
    }
}
