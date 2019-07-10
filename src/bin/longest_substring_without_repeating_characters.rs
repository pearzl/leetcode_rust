// 3
fn main() {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut maxlen = 0;
        let mut sl = vec![];
        for c in s.as_bytes() {
            let mut i = 0;
            for n in 0..sl.len() {
                if sl[n] == *c {
                    if sl.len() > maxlen {
                        maxlen = sl.len();
                    }
                    sl = sl.split_off(i + 1);
                    break;
                }
                i += 1;
            }
            sl.push(*c);
            // println!("{:?}", sl);
        }
        if sl.len() > maxlen {
            maxlen = sl.len();
        }
        maxlen as i32
    }
}
