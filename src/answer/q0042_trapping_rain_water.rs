// q0042_trapping_rain_water

struct Solution;

// impl Solution {
//     pub fn trap(height: Vec<i32>) -> i32 {
//         let mut lwall = vec!;
//         let mut rwalli = 0;
//         for (i, w) in height.iter().enumerate() {
//             if i < rwalli {
//                 continue;
//             }
//             if lwall.len() == 0 {
//                 lwall.push(i);
//                 continue;
//             }
//             if w <= heigth[lwall.last().unwrap()] {
//                 lwall.push(i);
//                 continue;
//             }
//             if w == height[lwall.last().unwrap()] {
//                 lwall.pop();
//                 lwall.push(i);
//                 continue;
//             }
//             if lwall.last().unwrap() + 1 >= i && lwall.len() == 1 {
//                 lwall.pop();
//                 lwall.push(i);
//                 continue;
//             }else {
//                 loop {
//                     let lwi = lwall.pop().unwrap();
//                     if height[lwi] >= w { // 往后找

//                     }else {

//                     }
//                 }
//             }
//         }
//     }
// }

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut walls = vec![];
        let mut from = 0;
        let mut ret = 0;
        while from < height.len() {
            if Solution::find_wall(&height, from, &mut walls) && walls.len() != 0 {
                // println!("{:?},{}", walls, from);
                while let Some((lw, rw)) = walls.pop() {
                    from = rw.max(from);
                    ret += (rw - lw - 1) as i32 * height[rw].min(height[lw]);
                    for i in lw + 1..rw {
                        ret -= height[i];
                    }
                }
            } else {
                from += 1;
            }
        }
        println!("contains water: {}", ret);
        ret
    }

    fn find_wall(height: &Vec<i32>, from: usize, ret: &mut Vec<(usize, usize)>) -> bool {
        let mut tmp = vec![];
        let mut first_wall = from;
        for i in from..height.len() {
            if height[i] != 0 {
                first_wall = i;
                break;
            }
        }
        // println!("first wall: {}", first_wall);
        for i in first_wall + 1..height.len() {
            if tmp.len() == 0 {
                if height[i] != 0 {
                    tmp.push(i);
                }
                continue;
            }
            if height[i] < height[*tmp.last().unwrap()] {
                continue;
            } else {
                tmp.push(i)
            }
        }
        // println!("{:?}", tmp);
        if tmp.len() == 0 {
            return false;
        }
        let mut build_wall_from = 0;
        for (l, i) in tmp.iter().enumerate() {
            if l == tmp.len() - 1 || height[first_wall] <= height[*i] {
                if first_wall + 1 < *i {
                    ret.push((first_wall, *i));
                    build_wall_from = l;
                }
                break;
            }
        }
        // println!("build wall from {}", build_wall_from);
        if build_wall_from < tmp.len() - 1 {
            for i in build_wall_from..tmp.len() - 1 {
                if tmp[i] + 1 < tmp[i + 1] {
                    ret.push((tmp[i], tmp[i + 1]));
                }
            }
        }
        // println!("{:?}", ret);
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        // assert_eq!( , );
        Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]);
        Solution::trap(vec![1, 2, 3, 7, 6, 4, 8, 3, 0, 5, 7, 4, 7, 4, 6, 4, 7]);
        Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 5, 2, 1, 1, 1, 1, 1]);
        Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 5, 3, 1, 1, 2, 1, 2]);
        // Solution::trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]);
        // Solution::trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]);
    }
}
