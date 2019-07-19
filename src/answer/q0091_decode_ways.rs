// q0091_decode_ways

struct Solution;

const VALID_CODE: [&str; 26] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17",
    "18", "19", "20", "21", "22", "23", "24", "25", "26",
];

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let slen = s.len();
        if slen == 0 {
            return 0;
        }
        let mut buf = vec![-1; slen + 1];
        buf[0] = 1;
        if VALID_CODE.contains(&&s[0..=0]) {
            buf[1] = 1;
        } else {
            return 0;
        }
        Solution::cal(&s, &mut buf, slen)
    }

    fn cal(s: &str, buf: &mut Vec<i32>, n: usize) -> i32 {
        if buf[n] != -1 {
            return buf[n];
        }
        let r1 = if VALID_CODE.contains(&&s[n - 1..=n - 1]) {
            Solution::cal(s, buf, n - 1)
        } else {
            0
        };
        let r2 = if VALID_CODE.contains(&&s[n - 2..=n - 1]) {
            Solution::cal(s, buf, n - 2)
        } else {
            0
        };
        buf[n] = r1 + r2;
        // println!("{:?}", buf);
        buf[n]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(0, Solution::num_decodings(String::from("")));
        assert_eq!(2, Solution::num_decodings(String::from("12")));
        assert_eq!(3, Solution::num_decodings(String::from("226")));
        assert_eq!(3, Solution::num_decodings(String::from("123532039485")));
        assert_eq!(
            0,
            Solution::num_decodings(String::from("122394857023532039485"))
        );
        assert_eq!(3, Solution::num_decodings(String::from("12385")));
        assert_eq!(8, Solution::num_decodings(String::from("1284691865913465")));
        assert_eq!( 1399680, Solution::num_decodings(String::from("1284691865912193749812020203458123421564323761861351246123512412512342341329573465")));
        assert_eq!( 283115520, Solution::num_decodings(String::from("12846918651827368253648176598713649876128975698173659813746519865987164876218746128756198765871346598136587613895761086510982651893651365873165876358686458276891219374981202020345812342156432376")));
    }
}
