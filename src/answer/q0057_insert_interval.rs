// q0057_insert_interval

struct Solution;

#[derive(PartialEq)]
enum State {
    S,
    M,
    I,
}

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut lb = new_interval[0];
        let mut rb = new_interval[1];
        let mut state = State::S;
        let mut ret = vec![];
        for itvl in intervals.into_iter() {
            match state {
                State::S => {
                    if rb < itvl[0] {
                        ret.push(vec![lb, rb]);
                        ret.push(itvl);
                        state = State::I;
                    } else if itvl[1] < lb {
                        ret.push(itvl);
                    } else {
                        state = State::M;
                        lb = itvl[0].min(lb);
                        rb = itvl[1].max(rb);
                    }
                }
                State::I => {
                    ret.push(itvl);
                }
                State::M => {
                    if itvl[0] <= rb {
                        rb = itvl[1].max(rb);
                    } else {
                        ret.push(vec![lb, rb]);
                        ret.push(itvl);
                        state = State::I;
                    }
                }
            }
        }
        if state != State::I {
            ret.push(vec![lb, rb]);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![vec![1, 5], vec![6, 9]],
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5])
        );
        assert_eq!(
            vec![vec![1, 2], vec![3, 10], vec![12, 16]],
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            )
        );
        assert_eq!(vec![vec![4, 8]], Solution::insert(vec![], vec![4, 8]));
    }
}
