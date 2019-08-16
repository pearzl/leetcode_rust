// q0126_word_ladder_ii

struct Solution;

impl Solution {
//     pub fn find_ladders(
//         begin_word: String,
//         end_word: String,
//         word_list: Vec<String>,
//     ) -> Vec<Vec<String>> {
//         if !word_list.contains(&end_word) {
//             return vec![];
//         }
//         if Solution::differ_one(&begin_word, &end_word) {
//             return vec![vec![begin_word, end_word]];
//         }
//         let mut ret = vec![];
//         for s in word_list.iter() {
//             if Solution::differ_one(s, &begin_word) {
//                 let new_word_list: Vec<String> = word_list
//                     .iter()
//                     .filter(|ss| ss != &s)
//                     .map(|ss| ss.clone())
//                     .collect();
//                 let r = Solution::find_ladders(s.clone(), end_word.clone(), new_word_list);
//                 if r.is_empty() {
//                     continue;
//                 }
//                 for mut ns in r.into_iter() {
//                     ns.insert(0, begin_word.clone());
//                     ret.push(ns);
//                 }
//             }
//         }
//         let mut min_len = usize::max_value();
//         let mut new_ret = vec![];
//         for v in ret.into_iter() {
//             let l = v.len();
//             if min_len > l {
//                 new_ret.clear();
//                 min_len = l;
//                 new_ret.push(v)
//             } else if min_len == l {
//                 new_ret.push(v)
//             }
//         }
//         new_ret
//     }
    pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
        if !word_list.contains(&end_word) {
            return vec![];
        }
        let mut searched_pos = vec![false; word_list.len()];
        let ret = Solution::search(begin_word, end_word, &word_list, &mut searched_pos);
        ret
    }

    fn search(begin_word: String, end_word: String, word_list: &[String], searched_pos: &mut [bool]) -> Vec<Vec<String>>{
        if Solution::differ_one(&begin_word, &end_word) {
            return vec![vec![begin_word, end_word]];
        }
        let mut ret = vec![];
        let mut ss = vec![];
        for (i, s) in word_list.iter().enumerate() {
            if searched_pos[i] {
                continue
            }
            if Solution::differ_one(s, &begin_word) {
                searched_pos[i] = true;
                ss.push(s.clone());
            }
        }
        for s in ss.into_iter() {
            let r = Solution::search(s.clone(), end_word.clone(), word_list, searched_pos);
            if r.is_empty() {
                continue;
            }
            for mut v in r.into_iter() {
                v.insert(0,begin_word.clone());
                ret.push(v);
            }
        }
        ret
    }

    pub fn differ_one(s1: &str, s2: &str) -> bool {
        let mut count = 0;
        for i in 0..s1.len() {
            if s1[i..=i] != s2[i..=i] {
                count += 1;
            }
        }
        count == 1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![
                vec![
                    String::from("hit"),
                    String::from("hot"),
                    String::from("dot"),
                    String::from("dog"),
                    String::from("cog")
                ],
                vec![
                    String::from("hit"),
                    String::from("hot"),
                    String::from("lot"),
                    String::from("log"),
                    String::from("cog")
                ]
            ],
            Solution::find_ladders(
                String::from("hit"),
                String::from("cog"),
                vec![
                    String::from("hot"),
                    String::from("dot"),
                    String::from("dog"),
                    String::from("lot"),
                    String::from("log"),
                    String::from("cog")
                ]
            )
        );

        assert_eq!(
            Vec::<Vec<String>>::new(),
            Solution::find_ladders(
                String::from("hit"),
                String::from("cog"),
                vec![
                    String::from("hot"),
                    String::from("dot"),
                    String::from("dog"),
                    String::from("lot"),
                    String::from("log")
                ]
            )
        );

    }
}
