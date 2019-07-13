// q0043_multiply_strings

struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }
        let mut sum = vec![];
        let bnum2 = num2.as_bytes();
        for (i, c1) in num1.as_bytes().iter().enumerate() {
            let mut tmp = vec![];
            for n in 0..num1.len() - i - 1 {
                tmp.push(0);
            }
            let mut overflow = 0;
            let mut j = bnum2.len() - 1;
            loop {
                let mut r = (bnum2[j] - b'0') * (c1 - b'0') + overflow;
                overflow = r / 10;
                r = r % 10;
                tmp.push(r);
                if j == 0 {
                    break;
                } else {
                    j -= 1;
                }
            }
            if overflow != 0 {
                tmp.push(overflow);
            }
            sum.push(tmp);
        }
        // println!("{:?}", sum);
        let mut ret = sum.pop().unwrap();
        while let Some(mut v) = sum.pop() {
            // println!("sum -{:?}\n    -{:?}", ret, v );
            std::mem::swap(&mut v, &mut ret);
            let mut overflow = 0;
            for (i, n) in v.iter().enumerate() {
                let mut r = n + ret[i] + overflow;
                overflow = r / 10;
                r = r % 10;
                ret[i] = r;
            }
            for i in v.len()..ret.len() {
                let mut r = ret[i] + overflow;
                overflow = r / 10;
                r = r % 10;
                ret[i] = r;
                if overflow == 0 {
                    break;
                }
            }
            if overflow != 0 {
                ret.push(overflow);
            }
            // println!("result: {:?}", ret);
        }
        // println!("{:?}", ret);
        let mut output = String::with_capacity(ret.len());
        while let Some(c) = ret.pop() {
            output.push((c + b'0') as char);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            "126673",
            Solution::multiply(String::from("113"), String::from("1121"))
        );
        assert_eq!(
            "0",
            Solution::multiply(String::from("0"), String::from("0"))
        );
        assert_eq!(
            "0",
            Solution::multiply(String::from("113"), String::from("0"))
        );
        assert_eq!(
            "1121",
            Solution::multiply(String::from("1"), String::from("1121"))
        );
        assert_eq!(
            "76049",
            Solution::multiply(String::from("113"), String::from("673"))
        );
        assert_eq!(
            "2897449555",
            Solution::multiply(String::from("2346113"), String::from("1235"))
        );
        assert_eq!(
            "15082182857135503",
            Solution::multiply(String::from("134513"), String::from("112124351231"))
        );
        assert_eq!(
            "3877856038537273",
            Solution::multiply(String::from("3451513"), String::from("1123523521"))
        );
        assert_eq!(
            "1037605098",
            Solution::multiply(String::from("113"), String::from("9182346"))
        );
        assert_eq!(
            "10512",
            Solution::multiply(String::from("584"), String::from("18"))
        );
    }
}
