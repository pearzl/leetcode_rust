// q0067_add_binary

struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut flag = 0;
        let mut tmp = vec![];
        let mut ai = a.chars().rev().fuse();
        let mut bi = b.chars().rev().fuse();
        loop {
            match (ai.next(), bi.next()) {
                (Some(ca), Some(cb)) => {
                    let sum = (ca as u8 - b'0') + (cb as u8 - b'0') + flag;
                    tmp.push(sum % 2);
                    flag = sum / 2;
                }
                (Some(ca), None) => {
                    let sum = (ca as u8 - b'0') + flag;
                    tmp.push(sum % 2);
                    flag = sum / 2;
                }
                (None, Some(cb)) => {
                    let sum = (cb as u8 - b'0') + flag;
                    tmp.push(sum % 2);
                    flag = sum / 2;
                }
                (None, None) => {
                    break;
                }
            }
        }
        if flag == 1 {
            tmp.push(1);
        }
        let mut ret = String::with_capacity(tmp.len());
        for c in tmp.iter().rev() {
            ret.push((c + b'0') as char);
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
            String::from("100"),
            Solution::add_binary(String::from("11"), String::from("1"))
        );
        assert_eq!(
            String::from("10101"),
            Solution::add_binary(String::from("1010"), String::from("1011"))
        );
        assert_eq!(
            String::from("0"),
            Solution::add_binary(String::from("0"), String::from("0"))
        );
        assert_eq!(
            String::from("10"),
            Solution::add_binary(String::from("1"), String::from("1"))
        );
    }
}
