// q0056_merge_intervals

struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() <= 1 {
            return intervals;
        }
        let mut intervals = intervals;
        Solution::sort(&mut intervals);
        // println!("after soring: {:?}", intervals);
        let mut ret = vec![];
        let mut tmp = vec![intervals[0][0], intervals[0][1]];
        for i in 1..intervals.len() {
            if tmp[0] < intervals[i][0] && tmp[1] < intervals[i][0] {
                ret.push(tmp.clone());
                tmp[0] = intervals[i][0];
                tmp[1] = intervals[i][1];
            } else {
                // tmp[0] = tmp[0].min(intervals[i][0]);
                tmp[1] = tmp[1].max(intervals[i][1]);
            }
        }
        ret.push(tmp);
        ret
    }

    pub fn sort(itvl: &mut [Vec<i32>]) {
        // println!("begin sort{:?}", itvl);
        let l = itvl.len();
        if l <= 1 {
            return;
        }
        let mut j = 0; //   假设pivot最小
        let pivot = itvl[l - 1][0];
        // println!("pivot is {:?}", itvl[l-1]);
        for i in 0..l - 1 {
            // print!("compare {} {:?}  ", i, itvl[i]);
            if itvl[i][0] < pivot {
                if i != j {
                    // print!("switch");
                    let t = std::mem::replace(&mut itvl[j], vec![]);
                    itvl[j] = std::mem::replace(&mut itvl[i], t);
                }
                j += 1;
            }
            // println!("");
        }
        if l - 1 != j {
            let t = std::mem::replace(&mut itvl[j], vec![]);
            itvl[j] = std::mem::replace(&mut itvl[l - 1], t);
        }
        // println!("sort {:?}", &itvl[0..j]);
        Solution::sort(&mut itvl[0..j]);
        // println!("after sort {:?}", &itvl[0..j]);
        // println!("sort {:?}", &itvl[j+1..]);
        Solution::sort(&mut itvl[j + 1..]);
        // println!("after sort {:?}", &itvl[j+1..]);
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![4, 5]]),
            vec![vec![1, 5]]
        );
        assert_eq!(
            Solution::merge(vec![
                vec![2, 3],
                vec![4, 5],
                vec![6, 7],
                vec![8, 9],
                vec![1, 10]
            ]),
            vec![vec![1, 10]]
        );
        let empty: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::merge(empty.clone()), empty);
    }

    #[test]
    #[ignore]
    fn sort() {
        let mut t = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        Solution::sort(&mut t);
        let r = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(r, t);

        // let mut t = vec![vec![2,3],vec![4,5],vec![6,7],vec![8,9],vec![1,10]];
        // Solution::sort(&mut t);
        // let r = vec![vec![1,10],vec![2,3],vec![4,5],vec![6,7],vec![8,9]];
        // assert_eq!(r, t);

        // let mut t = vec![vec![2,3],vec![4,5],vec![6,7],vec![8,9],vec![4,10]];
        // Solution::sort(&mut t);
        // let r = vec![vec![2,3],vec![4,5],vec![4,10],vec![6,7],vec![8,9]];
        // assert_eq!(r, t);

        // let mut t = vec![vec![2,3],vec![4,5],vec![6,7],vec![8,9],vec![1,10]];
        // Solution::sort(&mut t);
        // let r = vec![vec![1,10],vec![2,3],vec![4,5],vec![6,7],vec![8,9]];
        // assert_eq!(r, t);
    }

}
