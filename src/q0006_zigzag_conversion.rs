// q0006_zigzag_conversion

struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let mut bufv = vec![];
        for i in 0..num_rows {
            bufv.push(vec![]);
        }
        let mut l = 0;
        let mut flag: isize = -1;
        for c in s.as_bytes() {
            if l == bufv.len() - 1 || l == 0 {
                flag = -flag;
            }
            bufv[l].push(*c);
            l = (l as isize + flag) as usize;
        }
        let mut ret = vec![];
        for i in 0..bufv.len() {
            ret.append(&mut bufv[i]);
        }
        String::from_utf8(ret).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::convert(String::from("LEETCODEISHIRING"), 3),
            String::from("LCIRETOESIIGEDHN")
        );
    }
}
