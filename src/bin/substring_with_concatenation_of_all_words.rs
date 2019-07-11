
// 30

// pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
//     let mut i = 0;
//     let wnum = words.len();
//     let slen = s.len();
//     if wnum == 0 || slen == 0 {
//         return vec![];
//     }
//     let wlen = words[0].len();
//     if slen < wlen * wnum {
//         return vec![];
//     }
//     let mut ret = vec![];
//     while i <= slen - wlen * wnum {
//         // println!("search from: {}", i);
//         let mut si = i;
//         let mut invalid_pos = vec![];
//         while si < slen {
//             // println!("{},{}",si, si+wlen);
//             let tw = (&s[si..si+wlen]).to_string();
//             // print!("{}, {:?}", tw, invalid_pos);
//             match contains_on(tw, &words, &mut invalid_pos) {
//                 Some(n) => {
//                     invalid_pos.push(n);
//                     si += wlen;
//                     // println!("found on {}", n);
//                 },
//                 None => {
//                     // println!("   not found");
//                     break;
//                 }
//             }
//             if invalid_pos.len() == wnum {
//                 ret.push(i as i32);
//                 break;
//             }
//         }
//         // if invalid_pos.len() == wnum {
//         //     ret.push(i as i32);
//         // }
//         // println!("{},{}, {:?}, {}", i, slen - wlen * wnum, invalid_pos , wnum);
//         i += 1;
//     }
//     ret
// }

// fn contains_on(item: String, list: &Vec<String>, invalid_pos: &mut Vec<usize>) -> Option<usize> {
//     for (i, v) in list.iter().enumerate() {
//         if invalid_pos.contains(&i) {
//             continue;
//         }
//         if &item == v {
//             return Some(i);
//         }
//     }
//     None
// }

use std::collections::HashMap;

pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    let slen = s.len();
    let wnum = words.len();
    if slen == 0 || wnum == 0 {
        return vec![];
    }
    let wlen = words[0].len();
    let totlen = wlen * wnum;
    if wlen == 0 || slen < totlen {
        return vec![];
    }

    let mut sim = HashMap::new(); // how many times a word can use.
    let mut i = 0;
    let mut ret = vec![]; // result
    while i <= slen - totlen {
        // println!("searching from: {}", i);
        let mut si = i;
        let mut sic = HashMap::new(); // how many times a word already used.
        while si < slen {
            let tw = (&s[si..si+wlen]).to_string(); // current word.
            // print!("  {}", tw );
            match sim.get(&tw) {
                Some(n) => {
                    // print!("   , found on cache, max {}, ", n);
                    if *n == 0 {
                        break;
                    }
                    match sic.get_mut(&tw) {
                        Some(x) => {
                            // println!(" used {}", *x);
                            if *x < *n {
                                *x += 1;
                            }else {
                                break;
                            }
                        },
                        None => {
                            // println!(" used 0");
                            sic.insert(String::from(tw.as_str()), 1);
                        }
                    }
                },
                None => {
                    // print!("   , not found on cache, update. ");
                    let mut count = 0;
                    for w in words.iter() {
                        if *w == tw {
                            count += 1;
                        }
                    }
                    // println!(" max {}, used 0 ", count);
                    sim.insert(String::from(tw.as_str()), count);
                    if count == 0 {
                        break;
                    }
                    sic.insert(String::from(tw.as_str()), 1);
                }
            }
            // println!("end match");
            // println!("{},{},{},{}", si, wlen, i, totlen);
            if si+wlen-i == totlen {
                ret.push(i as i32);
                break;
            }
            si += wlen;
        }
        i += 1;
    }
    ret
}

fn main() {
    assert_eq!(vec![0,9], find_substring("barfoothefoobarman".to_string(), vec!["foo".to_string(), "bar".to_string()]) );
    assert_eq!(vec![8] as Vec<i32>, find_substring("wordgoodgoodgoodbestword".to_string(), vec!["word".to_string(), "good".to_string(), "best".to_string(), "good".to_string()]) );
    assert_eq!(vec![] as Vec<i32>, find_substring("wordgoodgoodgoodbestword".to_string(), vec!["word".to_string(), "good".to_string(), "best".to_string(), "word".to_string()]) );
    assert_eq!(vec![] as Vec<i32>, find_substring("".to_string(), vec![]) );
    assert_eq!(vec![] as Vec<i32>, find_substring("a".to_string(), vec!["a".to_string(), "a".to_string()]) );
    assert_eq!(vec![0,1,2] as Vec<i32>, find_substring("aaaaaaaa".to_string(), vec!["aa".to_string(), "aa".to_string(), "aa".to_string()]) );
    // let s = String::from("abababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababab");
    // let v: Vec<String> = vec!["ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba","ab","ba"].into_iter().map(|x| x.to_string()).collect();
    // assert_eq!(vec![0,1,2] as Vec<i32>, find_substring(s, v) );
}
