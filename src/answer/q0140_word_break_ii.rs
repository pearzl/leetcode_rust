// q0140_word_break_ii

struct Solution;

// use std::collections::HashSet;
// impl Solution {
//     pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
//         let l = word_dict.len();
//         let mut words_len = HashSet::with_capacity(l);
//         let mut words_dict = HashSet::with_capacity(l);
//         for s in word_dict.into_iter() {
//             words_len.insert(s.len());
//             words_dict.insert(s);
//         }
//         let ss = s.as_str();
//         let ssl = ss.len();
//         let r = Solution::break_word(&ss, &words_len, &words_dict);
//         let rl = r.len();
//         let mut ret = Vec::with_capacity(rl);
//         for sv in r.into_iter() {
//             let mut s = String::with_capacity(ssl + rl);
//             for w in sv.into_iter() {
//                 s.push_str(w);
//                 s.push(' ');
//             }
//             s.pop();
//             ret.push(s);
//         }
//         ret
//     }

//     fn break_word<'a>(
//         s: &'a str,
//         words_len: &HashSet<usize>,
//         words_dict: &HashSet<String>,
//     ) -> Vec<Vec<&'a str>> {
//         let mut ret = vec![];
//         let l = s.len();
//         for wl in words_len.iter().cloned() {
//             if wl < l && words_dict.contains(&s[0..wl]) {
//                 let v = Solution::break_word(&s[wl..], words_len, words_dict);
//                 for mut vs in v.into_iter() {
//                     vs.insert(0, &s[0..wl]);
//                     ret.push(vs);
//                 }
//             }else if wl == l && words_dict.contains(&s[0..wl]){
//                 ret.push(vec![s]);
//             }
//         }
//         ret
//     }
// }

// use std::collections::HashSet;
// use std::collections::HashMap;
// impl Solution {
//     pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
//         let slen = s.len();
//         let mut ret_sv: HashMap<usize, Vec<Vec<&str>>> = HashMap::with_capacity(slen);
//         let wdl = word_dict.len();
//         let mut words_len = HashSet::with_capacity(wdl);
//         let mut words_dict = HashSet::with_capacity(wdl);
//         for s in word_dict.into_iter() {
//             words_len.insert(s.len());
//             words_dict.insert(s);
//         }
//         for i in 1..=slen {
//             let mut sv = vec![];
//             let cur_str = &s[0..i];
//             println!("------------\n{}:", cur_str);
//             for wl in words_len.iter().cloned() {
//                 if i < wl {
//                     continue
//                 }else if i == wl {
//                     if words_dict.contains(cur_str) {
//                         sv.push(vec![cur_str]);
//                     }
//                 }else {
//                     if words_dict.contains(&s[i-wl..i]) {
//                         // println!("  contains {}", &s[i-wl..i]);
//                         if let Some(nsv) = ret_sv.get(&(i-wl)){
//                             for mut item in nsv.clone().into_iter() {
//                                 item.push(&s[i-wl..i]);
//                                 sv.push(item);
//                             }
//                         }
//                     }
//                 }
//             }
//             println!("{:?}", sv);
//             ret_sv.insert(i, sv);
//         }
//         for i in ret_sv.iter() {
//             println!("--{:?}", i);
//         }
//         let ret_sv_last = ret_sv.get(&slen).unwrap();
//         let rl = ret_sv_last.len();
//         let mut ret = Vec::with_capacity(rl);
//         for sv in ret_sv_last.into_iter() {
//             let mut s = String::with_capacity(slen + rl);
//             for w in sv.into_iter() {
//                 s.push_str(w);
//                 s.push(' ');
//             }
//             s.pop();
//             ret.push(s);
//         }
//         ret
//     }
// }

use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let slen = s.len();
        let mut ret_sv: HashMap<usize, Vec<(usize, &str)>> = HashMap::with_capacity(slen);
        let wdl = word_dict.len();
        let mut words_len = HashSet::with_capacity(wdl);
        let mut words_dict = HashSet::with_capacity(wdl);
        for s in word_dict.into_iter() {
            words_len.insert(s.len());
            words_dict.insert(s);
        }
        for i in 1..=slen {
            let mut sv = vec![];
            let cur_str = &s[0..i];
            for wl in words_len.iter().cloned() {
                if i < wl {
                    continue;
                } else if i == wl {
                    if words_dict.contains(cur_str) {
                        sv.push((0, cur_str));
                    }
                } else {
                    if words_dict.contains(&s[i - wl..i]) {
                        if ret_sv.get(&(i - wl)).is_some() {
                            sv.push((i - wl, &s[i - wl..i]));
                        }
                    }
                }
            }
            if !sv.is_empty() {
                ret_sv.insert(i, sv);
            }
        }
        if ret_sv.get(&slen).is_some() {
            let mut dp: HashMap<usize, Vec<String>> = HashMap::new();
            for i in 1..=slen {
                if let Some(x) = ret_sv.get(&i) {
                    let mut r = vec![];
                    for (pi, ss) in x.into_iter() {
                        if *pi == 0 {
                            r.push(ss.to_string());
                        } else {
                            for nss in dp.get(pi).unwrap().iter() {
                                let mut ns = nss.clone();
                                ns.push(' ');
                                ns.push_str(ss);
                                r.push(ns);
                            }
                        }
                    }
                    dp.insert(i, r);
                }
            }
            return dp.get(&slen).unwrap().clone();
        } else {
            return vec![];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    use crate::util;

    #[test]
    fn it_works() {
        assert_eq!(
            util::vec_2_set(util::build_string_array(vec![
                "cats and dog",
                "cat sand dog"
            ])),
            util::vec_2_set(Solution::word_break(
                String::from("catsanddog"),
                util::build_string_array(vec!["cat", "cats", "and", "sand", "dog"])
            ))
        );

        assert_eq!(
            util::vec_2_set(util::build_string_array(vec![
                "pine apple pen apple",
                "pineapple pen apple",
                "pine applepen apple"
            ])),
            util::vec_2_set(Solution::word_break(
                String::from("pineapplepenapple"),
                util::build_string_array(vec!["apple", "pen", "applepen", "pine", "pineapple"])
            ))
        );

        assert_eq!(
            util::vec_2_set(util::build_string_array(vec![])),
            util::vec_2_set(Solution::word_break(
                String::from("catsandog"),
                util::build_string_array(vec!["cats", "dog", "sand", "and", "cat"])
            ))
        );

        assert_eq!(
            util::vec_2_set(util::build_string_array(vec![])),
            util::vec_2_set(Solution::word_break(
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                util::build_string_array(vec!["a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"])
            ))
        );

        assert_eq!(
            util::vec_2_set(util::build_string_array(vec![
                "a a a a a a a",
                "a a a a a aa",
                "a a a a aa a",
                "a a a aa a a",
                "a a a aa aa",
                "a a a aaaa",
                "a a aa a a a",
                "a a aa a aa",
                "a a aa aa a",
                "a a aaaa a",
                "a aa a a a a",
                "a aa a a aa",
                "a aa a aa a",
                "a aa aa a a",
                "a aa aa aa",
                "a aa aaaa",
                "a aaaa a a",
                "a aaaa aa",
                "aa a a a a a",
                "aa a a a aa",
                "aa a a aa a",
                "aa a aa a a",
                "aa a aa aa",
                "aa a aaaa",
                "aa aa a a a",
                "aa aa a aa",
                "aa aa aa a",
                "aa aaaa a",
                "aaaa a a a",
                "aaaa a aa",
                "aaaa aa a"
            ])),
            util::vec_2_set(Solution::word_break(
                String::from("aaaaaaa"),
                util::build_string_array(vec!["aaaa", "aa", "a"])
            ))
        );
    }
}
