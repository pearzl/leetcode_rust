// q0077_combinations

struct Solution;

// impl Solution {
//     pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
//         let mut ret = vec![];
//         let ref mut tmpv = vec![];
//         let ref mut picked = vec![false; n as usize];
//         Solution::c(n, k, picked, tmpv, &mut ret);
//         ret
//     }

//     fn c(n: i32, k: i32, picked: &mut Vec<bool>, tmpv: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
//         if k == 0 {
//             let mut t = tmpv.clone();
//             t.sort_unstable();
//             if !ret.contains(&t) {
//                 ret.push(t);
//             }
//             return;
//         }
//         if !picked.contains(&false) {
//             return;
//         }
//         for i in 0..n as usize {
//             if picked[i] {
//                 continue;
//             }
//             picked[i] = true;
//             tmpv.push((i + 1) as i32);
//             Solution::c(n, k - 1, picked, tmpv, ret);
//             tmpv.pop();
//             picked[i] = false;
//         }
//     }
// }

// impl Solution {
//     pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
//         if n < k {
//             return vec![];
//         }
//         let mut ret: Vec<(Vec<bool>, Vec<i32>)> = vec![(vec![false; n as usize],vec![])];
//         for _ in 0..k {
//             let mut tmp: Vec<(Vec<bool>, Vec<i32>)> = vec![];
//             for r in ret.iter() {
//                 for i in 0..n as usize {
//                     if r.0[i] {
//                         continue;
//                     }
//                     let mut t1 = r.1.clone();
//                     let mut t0 = r.0.clone();
//                     t0[i] = true;
//                     t1.push(i as i32 + 1);
//                     t1.sort_unstable();
//                     let t = (t0, t1);
//                     if !tmp.contains(&t) {
//                         tmp.push(t);
//                     }
//                 }
//             }
//             std::mem::swap(&mut ret, &mut tmp);
//         }
//         let r: Vec<Vec<i32>> = ret.into_iter().map(|x| x.1).collect();
//         r
//     }
// }

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        if k == 0 {
            return vec![vec![]];
        }
        if n < k {
            return vec![];
        }
        let mut ret = vec![];
        for rb in k..=n {
            let mut init: Vec<i32> = (1..k).into_iter().collect();
            init.push(rb);
            let mut tmpr = vec![];
            let mut i = init.len() - 1;
            tmpr.push(init);
            while i > 0 {
                for index in 0..tmpr.len() {
                    let item = tmpr[index].clone();
                    let mut j = 1;
                    while item[i] != item[i - 1] + j {
                        let mut t = item.clone();
                        t[i - 1] += j;
                        tmpr.push(t);
                        j += 1;
                    }
                }
                i -= 1;
            }
            ret.append(&mut tmpr);
            tmpr.clear();
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    use crate::util;

    #[test]
    fn it_works() {
        let empty_item: Vec<i32> = vec![];
        let empty_ret: Vec<Vec<i32>> = vec![];
        assert_eq!(vec![empty_item.clone()], Solution::combine(0, 0));
        assert_eq!(vec![empty_item.clone()], Solution::combine(1, 0));
        assert_eq!(empty_ret, Solution::combine(0, 1));
        assert_eq!(
            util::vec_2_set(vec![
                vec![2, 4],
                vec![3, 4],
                vec![2, 3],
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
            ]),
            util::vec_2_set(Solution::combine(4, 2))
        );
        assert_eq!(
            vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]],
            Solution::combine(13, 13)
        );
        let v30: Vec<i32> = (1..=30).into_iter().collect();
        assert_eq!(vec![v30], Solution::combine(30, 30));
        assert_eq!(
            util::vec_2_set(vec![
                vec![1, 2, 3],
                vec![1, 2, 4],
                vec![1, 2, 5],
                vec![1, 3, 4],
                vec![1, 3, 5],
                vec![1, 4, 5],
                vec![2, 3, 4],
                vec![2, 3, 5],
                vec![2, 4, 5],
                vec![3, 4, 5]
            ]),
            util::vec_2_set(Solution::combine(5, 3))
        );
        assert_eq!(
            util::vec_2_set(vec![
                vec![1, 2, 3],
                vec![1, 2, 4],
                vec![1, 2, 5],
                vec![1, 2, 6],
                vec![1, 2, 7],
                vec![1, 3, 4],
                vec![1, 3, 5],
                vec![1, 3, 6],
                vec![1, 3, 7],
                vec![1, 4, 5],
                vec![1, 4, 6],
                vec![1, 4, 7],
                vec![1, 5, 6],
                vec![1, 5, 7],
                vec![1, 6, 7],
                vec![2, 3, 4],
                vec![2, 3, 5],
                vec![2, 3, 6],
                vec![2, 3, 7],
                vec![2, 4, 5],
                vec![2, 4, 6],
                vec![2, 4, 7],
                vec![2, 5, 6],
                vec![2, 5, 7],
                vec![2, 6, 7],
                vec![3, 4, 5],
                vec![3, 4, 6],
                vec![3, 4, 7],
                vec![3, 5, 6],
                vec![3, 5, 7],
                vec![3, 6, 7],
                vec![4, 5, 6],
                vec![4, 5, 7],
                vec![4, 6, 7],
                vec![5, 6, 7]
            ]),
            util::vec_2_set(Solution::combine(7, 3))
        );
    }
}
