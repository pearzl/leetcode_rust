// q0089_gray_code

struct Solution;

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        if n == 0 {
            return vec![0];
        }
        let mut ret = Solution::gray_code(n - 1);
        let rev_bit = 1 << (n - 1);
        let mut rev_ret: Vec<i32> = ret
            .iter()
            .rev()
            .map(|x| {
                let t = *x ^ rev_bit;
                t
            })
            .collect();
        ret.append(&mut rev_ret);
        println!("{:?}", ret);
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        // assert!(is_gray_code(Solution::gray_code(0)));
        assert!(is_gray_code(Solution::gray_code(20)));

        // assert!(is_gray_code(vec![0,1,3,2]));
        // assert!(is_gray_code(vec![0,2,3,1]));
    }

    fn is_gray_code(code: Vec<i32>) -> bool {
        if code.len() == 0 {
            return false;
        }
        if code[0] != 0 {
            return false;
        }
        for i in 1..code.len() {
            let bit_compare = code[i - 1] ^ code[i];
            let one_count = bit_compare.count_ones();
            if one_count != 1 {
                return false;
            }
        }
        return true;
    }
}
