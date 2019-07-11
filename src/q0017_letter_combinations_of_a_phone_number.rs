// q0017_letter_combinations_of_a_phone_number

struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let map = vec![
            vec![b'a', b'b', b'c'],
            vec![b'd', b'e', b'f'],
            vec![b'g', b'h', b'i'],
            vec![b'j', b'k', b'l'],
            vec![b'm', b'n', b'o'],
            vec![b'p', b'q', b'r', b's'],
            vec![b't', b'u', b'v'],
            vec![b'w', b'x', b'y', b'z'],
        ];
        let mut ret = vec![];
        for d in digits.as_bytes() {
            if *d < b'2' || *d > b'9' {
                return ret;
            }
            let k = *d - b'2';
            if ret.len() == 0 {
                for i in map.get(k as usize).unwrap().iter() {
                    ret.push((*i as char).to_string());
                }
                continue;
            }
            let mut new_ret = vec![];
            for i in ret.iter() {
                for j in map.get(k as usize).unwrap().iter() {
                    new_ret.push(format!("{}{}", i, *j as char));
                }
            }
            ret = new_ret;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"],
            Solution::letter_combinations("23".to_string())
        );
    }
}
