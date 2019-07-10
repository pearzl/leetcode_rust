// 8
pub fn my_atoi(str: String) -> i32 {
    let mut ret: i32 = 0;
    let mut neg = false;
    let s = str.as_bytes();
    let mut index = 0;
    for &c in s {
        if c >= b'0' && c <= b'9' {
            break;
        } else if c == b'-' {
            neg = true;
            index += 1;
            break;
        } else if c == b'+' {
            neg = false;
            index += 1;
            break;
        } else if c == b' ' {
            index += 1;
        } else {
            return 0;
        }
    }
    for &c in &s[index..] {
        if c < b'0' || c > b'9' {
            break;
        }
        if neg {
            match ret.checked_mul(10) {
                Some(n) => ret = n,
                None => return i32::min_value(),
            }
            match ret.checked_sub((c - b'0') as i32) {
                Some(n) => ret = n,
                None => return i32::min_value(),
            }
        } else {
            match ret.checked_mul(10) {
                Some(n) => ret = n,
                None => return i32::max_value(),
            }
            match ret.checked_add((c - b'0') as i32) {
                Some(n) => ret = n,
                None => return i32::max_value(),
            }
        }
    }
    ret
}
