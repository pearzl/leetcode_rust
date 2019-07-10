// 7
pub fn reverse(x: i32) -> i32 {
    let mut n: i32 = 0;
    let mut x = x;
    loop {
        let b = x % 10;
        if x == 0 && b == 0 {
            break;
        }
        if let Some(t1) = n.checked_mul(10) {
            if let Some(t2) = t1.checked_add(b) {
                n = t2;
            } else {
                return 0;
            }
        } else {
            return 0;
        }
        x /= 10;
    }
    n
}
