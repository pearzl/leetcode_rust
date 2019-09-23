// q0216_combination_sum_iii

struct Solution;

// impl Solution {
//     pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
//         if (1+k)*k / 2 > n {
//             return vec![];
//         }
//         if k == 1 {
//             if n > 0 && n < 10 {
//                 return vec![vec![n]];
//             }else {
//                 return vec![];
//             }
//         }
//         let mut ret = vec![];
//         for i in 1..=9 {
//             for mut r in Solution::combination_sum3(k-1, n-i) {
//                 if r.contains(&i) {
//                     continue;
//                 }
//                 r.push(i);
//                 r.sort_unstable();
//                 if ret.contains(&r) {
//                     continue;
//                 }
//                 ret.push(r);
//             }
//         }
//         ret
//     }
// }

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let bottom = (1 + k) * k / 2;
        if bottom > n {
            return vec![];
        }
        let top = (9 + 10 - k) * k / 2;
        if top < n {
            return vec![];
        }

        let mut ret: Vec<Vec<i32>> = vec![(1..=k).into_iter().collect()];

        let k = k as usize;
        for _ in 0..n - bottom {
            let mut tmp = vec![];
            for v in ret.iter() {
                if v[k - 1] != 9 {
                    let mut t = v.clone();
                    t[k - 1] += 1;
                    if !tmp.contains(&t) {
                        tmp.push(t);
                    }
                }
                for i in 2..=k {
                    if v[k - i] != 9 && v[k - i + 1] - v[k - i] > 1 {
                        let mut t = v.clone();
                        t[k - i] += 1;
                        if !tmp.contains(&t) {
                            tmp.push(t);
                        }
                    }
                }
            }
            // println!("{:?}-----{:?}", ret, tmp);
            std::mem::swap(&mut ret, &mut tmp);
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        // assert_eq!( vec![vec![1]], Solution::combination_sum3(1, 1));
        // assert_eq!( Vec::<Vec<i32>>::new(), Solution::combination_sum3(1, 18));
        // assert_eq!( vec![vec![1,2,4]], Solution::combination_sum3(3, 7));
        // assert_eq!( vec![vec![1,2,3,4,5,6,7,8,9]], Solution::combination_sum3(9, 45));
        assert_eq!(
            vec![vec![1, 5], vec![2, 4]],
            Solution::combination_sum3(2, 6)
        );
    }
}
