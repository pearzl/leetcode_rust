// q0072_edit_distance

struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut buf = (0..word1.len()).map(|_| vec![-1; word2.len()]).collect();
        Solution::md(&word1, &word2, &mut buf)
    }

    fn md(word1: &str, word2: &str, buf: &mut Vec<Vec<i32>>) -> i32 {
        let wl1 = word1.len();
        let wl2 = word2.len();
        if wl1 == 0 && wl2 == 0 {
            return 0;
        } else if wl1 == 0 {
            return wl2 as i32;
        } else if wl2 == 0 {
            return wl1 as i32;
        } else {
            if buf[wl1 - 1][wl2 - 1] != -1 {
                return buf[wl1 - 1][wl2 - 1];
            }
            let insert = Solution::md(&word1[0..wl1 - 1], &word2[..], buf);
            let delete = Solution::md(&word1[..], &word2[0..wl2 - 1], buf);
            let modify = Solution::md(&word1[0..wl1 - 1], &word2[0..wl2 - 1], buf);
            if &word1[wl1 - 1..] == &word2[wl2 - 1..] {
                buf[wl1 - 1][wl2 - 1] = modify.min(insert + 1).min(delete + 1);
            } else {
                buf[wl1 - 1][wl2 - 1] = (modify + 1).min(insert + 1).min(delete + 1);
            }
            return buf[wl1 - 1][wl2 - 1];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            3,
            Solution::min_distance(String::from("horse"), String::from("ros"))
        );
        assert_eq!(
            5,
            Solution::min_distance(String::from("intention"), String::from("execution"))
        );
        assert_eq!(
            0,
            Solution::min_distance(String::from("a"), String::from("a"))
        );
        assert_eq!(
            1,
            Solution::min_distance(String::from("a"), String::from("ab"))
        );
        assert_eq!(
            6,
            Solution::min_distance(String::from("asdfasdfasdf"), String::from("sadfas"))
        );
        assert_eq!(
            6,
            Solution::min_distance(
                String::from("dinitrophenylhydrazine"),
                String::from("acetylphenylhydrazine")
            )
        );
    }

}
