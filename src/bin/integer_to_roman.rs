// 12
fn main() {
    assert_eq!("V", int_to_roman(5));
    assert_eq!("IV", int_to_roman(4));
    assert_eq!("V", int_to_roman(5));
    assert_eq!("V", int_to_roman(5));
    assert_eq!("V", int_to_roman(5));
    assert_eq!("IXXLCMMMM", int_to_roman(3999));
}

pub fn int_to_roman(num: i32) -> String {
    let c = [
        "I", "IV", "V", "IX", "X", "XL", "L", "XC", "C", "CD", "D", "CM", "M",
    ];
    let n = [1, 4, 5, 9, 10, 40, 50, 90, 100, 400, 500, 900, 1000];
    // for i in c.iter().zip(n.iter()) {
    //     println!("{:?}", i);
    // }

    let mut ret = vec![];
    let mut num = num;
    let mut i = n.len() - 1;
    while num > 0 {
        if num >= n[i] {
            ret.push(c[i]);
            num -= n[i]
        } else {
            i -= 1;
        }
    }
    let mut s = String::new();
    for c in ret.iter() {
        s.push_str(c)
    }
    s
}
