// q0068_text_justification

struct Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut cur_line_len = 0;
        let max_width = max_width as usize;
        let mut line = vec![];
        let mut ret = vec![];
        for w in words.into_iter() {
            let cur_word_len = w.len();
            let new_line_line = if cur_line_len == 0 {
                cur_line_len + cur_word_len
            } else {
                cur_line_len + cur_word_len + 1
            };
            if new_line_line > max_width {
                ret.push(Solution::build_string(false, &line, max_width));
                cur_line_len = cur_word_len;
                line.clear();
                line.push(w)
            } else {
                line.push(w);
                cur_line_len = new_line_line;
            }
        }
        ret.push(Solution::build_string(true, &line, max_width));
        ret
    }

    pub fn build_string(last_line: bool, words: &Vec<String>, max_width: usize) -> String {
        let wlen = words.len();
        let mut ret = String::with_capacity(max_width);
        if wlen == 0 {
            for _ in 0..max_width {
                ret.push(' ');
            }
            return ret;
        }
        if wlen == 1 {
            ret.push_str(&words[0]);
            for _ in ret.len()..max_width {
                ret.push(' ');
            }
            return ret;
        }
        if !last_line {
            let space_num = wlen - 1;
            let words_len: usize = words.iter().map(|s| s.len()).sum();
            let avg_space_len = (max_width - words_len) / space_num;
            let more_space_num = (max_width - words_len) % space_num;
            for (i, word) in words.iter().enumerate() {
                ret.push_str(word);
                if i == words.len() - 1 {
                    break;
                }
                for _ in 0..avg_space_len {
                    ret.push(' ');
                }
                if i < more_space_num {
                    ret.push(' ');
                }
            }
            return ret;
        } else {
            let mut ret = String::with_capacity(max_width);
            for (i, word) in words.iter().enumerate() {
                ret.push_str(word);
                if i == words.len() - 1 {
                    break;
                }
                ret.push(' ');
            }
            for _ in ret.len()..max_width as usize {
                ret.push(' ');
            }
            return ret;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![
                String::from("This    is    an"),
                String::from("example  of text"),
                String::from("justification.  ")
            ],
            Solution::full_justify(
                vec![
                    String::from("This"),
                    String::from("is"),
                    String::from("an"),
                    String::from("example"),
                    String::from("of"),
                    String::from("text"),
                    String::from("justification.")
                ],
                16
            )
        );

        let input: Vec<String> = [
            "Science",
            "is",
            "what",
            "we",
            "understand",
            "well",
            "enough",
            "to",
            "explain",
            "to",
            "a",
            "computer.",
            "Art",
            "is",
            "everything",
            "else",
            "we",
            "do",
        ]
        .into_iter()
        .map(|s| String::from(*s))
        .collect();
        let output: Vec<String> = [
            "Science  is  what we",
            "understand      well",
            "enough to explain to",
            "a  computer.  Art is",
            "everything  else  we",
            "do                  ",
        ]
        .into_iter()
        .map(|s| String::from(*s))
        .collect();
        assert_eq!(output, Solution::full_justify(input, 20));

        assert_eq!(
            vec![String::from("          ")],
            Solution::full_justify(vec![String::from("")], 10)
        );

        assert_eq!(
            vec![String::from("         a"), String::from("sflsjfljs ")],
            Solution::full_justify(
                vec![
                    String::from(""),
                    String::from("a"),
                    String::from("sflsjfljs ")
                ],
                10
            )
        );

        assert_eq!(
            vec![String::from("a")],
            Solution::full_justify(vec![String::from("a")], 1)
        );

        let empty: Vec<String> = vec![];
        assert_eq!(vec![String::from(" ")], Solution::full_justify(empty, 1));

        let input: Vec<String> = ["What", "must", "be", "acknowledgment", "shall", "be"]
            .into_iter()
            .map(|s| String::from(*s))
            .collect();
        let output: Vec<String> = ["What   must   be", "acknowledgment  ", "shall be        "]
            .into_iter()
            .map(|s| String::from(*s))
            .collect();
        assert_eq!(output, Solution::full_justify(input, 16));
    }

    #[test]
    fn build_string() {
        assert_eq!(
            "we are   ",
            Solution::build_string(true, &mut vec![String::from("we"), String::from("are")], 9)
        );
        assert_eq!(
            "we are good",
            Solution::build_string(
                true,
                &mut vec![
                    String::from("we"),
                    String::from("are"),
                    String::from("good")
                ],
                11
            )
        );
        assert_eq!(
            "hello     world",
            Solution::build_string(
                false,
                &mut vec![String::from("hello"), String::from("world")],
                15
            )
        );
        assert_eq!(
            "hello world hello rust",
            Solution::build_string(
                false,
                &mut vec![
                    String::from("hello"),
                    String::from("world"),
                    String::from("hello"),
                    String::from("rust")
                ],
                22
            )
        );
        assert_eq!(
            "hello  world hello rust",
            Solution::build_string(
                false,
                &mut vec![
                    String::from("hello"),
                    String::from("world"),
                    String::from("hello"),
                    String::from("rust")
                ],
                23
            )
        );
    }
}
