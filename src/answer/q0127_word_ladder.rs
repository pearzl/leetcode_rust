// q0127_word_ladder

struct Solution;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        if !word_list.contains(&end_word) {
            return 0;
        }
        let mut searched_pos = vec![false; word_list.len()];
        let mut begin_words = vec![&begin_word];
        let mut count = 1;
        loop {
            let mut new_begin_words = vec![];
            for bw in begin_words.into_iter() {
                for (i, w) in word_list.iter().enumerate() {
                    if searched_pos[i] {
                        continue;
                    }
                    if Solution::differ_one(w, bw) {
                        new_begin_words.push(w);
                        searched_pos[i] = true;
                    }
                }
            }
            if new_begin_words.is_empty() {
                return 0;
            }
            count += 1;
            if new_begin_words.contains(&&end_word) {
                return count;
            } else {
                begin_words = new_begin_words;
            }
        }
    }

    fn differ_one(s1: &str, s2: &str) -> bool {
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
            0,
            Solution::ladder_length(
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
        assert_eq!(
            5,
            Solution::ladder_length(
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
    }
}
