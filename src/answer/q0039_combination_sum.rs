// q0039_combination_sum

struct Solution;

// impl Solution {
//     pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
//         let mut ret = vec![];
//         Solution::solve(&candidates, target, &mut ret);
//         ret
//     }

//     fn solve(cddt: &[i32], target: i32, ret: &mut Vec<Vec<i32>>) {
//         for i in 1..=cddt.len() {
//             let mut pos_comb = vec![];
//             Solution::combine(&cddt, i as i32, vec![], &mut pos_comb);
//             for scomb in pos_comb.iter() {
//                 let sum: i32 = scomb.iter().sum();
//                 if sum == target {
//                     ret.push(scomb.clone());
//                 } else if sum < target {
//                     let mut newr = Solution::combination_sum(scomb.clone(), target - sum);
//                     while let Some(mut v) = newr.pop() {
//                         for n in scomb {
//                             v.push(*n);
//                         }
//                         ret.push(v);
//                     }
//                 }
//             }
//         }
//     }

//     pub fn combine(c: &[i32], n: i32, mut selected: Vec<i32>, ret: &mut Vec<Vec<i32>>) {
//         if 0 == n {
//             ret.push(selected);
//             return;
//         }
//         let clen = c.len();
//         if clen < n as usize {
//             return;
//         }
//         let mut y = selected.clone();
//         if clen == 1 {
//             selected.push(c[0]);
//             ret.push(selected);
//             return;
//         }
//         y.push(c[0]);
//         Solution::combine(&c[1..], n - 1, y, ret);
//         Solution::combine(&c[1..], n, selected, ret);
//     }
// }


impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort_unstable();
        println!("{:?}", candidates);
        let mut ret = vec![];
        let mut tmp = vec![];
        Solution::solve(&candidates[..], target, &mut tmp, &mut ret);
        ret
    }

    fn solve(cddt: &[i32], target: i32, tmp: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
        println!("begin with  {:?}", tmp);
        for (index, i) in cddt.iter().enumerate() {
            tmp.push(*i);
            let sum: i32 = tmp.iter().sum();
            println!("working on  {:?}--{}", tmp, sum);
            if sum < target {
                Solution::solve(&cddt[index..], target, tmp, ret);
                // tmp.pop();
            }else if sum == target {
                println!("hit!");
                ret.push(tmp.clone());
                tmp.pop();
                return
            }else {
                tmp.pop();
                return
            }
            tmp.pop();
            println!("end with    {:?}", tmp);
        }
        println!("solve end  {:?}", tmp);
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        // assert_eq!(
        //     Solution::combination_sum(vec![2, 3, 6, 7], 7),
        //     vec![vec![7], vec![2, 2, 3]]
        // );
        // assert_eq!(
        //     Solution::combination_sum(vec![2, 3, 7], 18),
        //     vec![vec![2,2,2,2,2,2,2,2,2],vec![2,2,2,2,2,2,3,3],vec![2,2,2,2,3,7],vec![2,2,2,3,3,3,3],vec![2,2,7,7],vec![2,3,3,3,7],vec![3,3,3,3,3,3]]
        // );
        // assert_eq!(
        //     Solution::combination_sum(vec![2, 3, 5], 8),
        //     vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        // );
        assert_eq!(
            Solution::combination_sum(
                vec![
                    48, 22, 49, 24, 26, 47, 33, 40, 37, 39, 31, 46, 36, 43, 45, 34, 28, 20, 29, 25,
                    41, 32, 23
                ],
                69
            ),
            vec![
                vec![20, 20, 29],
                vec![20, 23, 26],
                vec![20, 24, 25],
                vec![20, 49],
                vec![22, 22, 25],
                vec![22, 23, 24],
                vec![22, 47],
                vec![23, 23, 23],
                vec![23, 46],
                vec![24, 45],
                vec![26, 43],
                vec![28, 41],
                vec![29, 40],
                vec![32, 37],
                vec![33, 36]
            ]
        );
    }

    // #[test]
    // #[ignore]
    // fn test_conbine() {
    //     let mut ret = vec![];
    //     let src = vec![1, 2, 3, 4, 5];
    //     let tmp = vec![];
    //     Solution::combine(&src, 2, tmp, &mut ret);
    //     assert_eq!(
    //         ret,
    //         vec![
    //             vec![1, 2],
    //             vec![1, 3],
    //             vec![1, 4],
    //             vec![1, 5],
    //             vec![2, 3],
    //             vec![2, 4],
    //             vec![2, 5],
    //             vec![3, 4],
    //             vec![3, 5],
    //             vec![4, 5],
    //         ]
    //     );

        // let mut ret = vec![];
        // let src = vec![1,2,3,4,5];
        // let tmp = vec![];
        // Solution::combine(&src, 1, tmp, &mut ret);
        // assert_eq!(ret, vec![
        //     vec![2],
        //     vec![3],
        //     vec![4],
        //     vec![5],
        //     vec![1],
        // ]);
    // }
}
