// q0168_excel_sheet_column_title

struct Solution;

impl Solution {
    const BASE: [char; 26] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    pub fn convert_to_title(n: i32) -> String {
        let base = &Solution::BASE;
        let mut n = n as usize;
        let mut ret = vec![];
        while n > 26 {
            ret.push(base[(n - 1) % 26]);
            n = (n - 1) / 26;
        }
        let mut s = format!("{}", base[n - 1]);
        while let Some(b) = ret.pop() {
            s.push(b);
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(String::from("A"), Solution::convert_to_title(1));
        assert_eq!(String::from("AB"), Solution::convert_to_title(28));
        assert_eq!(String::from("AA"), Solution::convert_to_title(27));
        assert_eq!(String::from("Z"), Solution::convert_to_title(26));
        assert_eq!(String::from("AZ"), Solution::convert_to_title(52));
        assert_eq!(String::from("ZY"), Solution::convert_to_title(701));
    }
}
