// 15

fn main() {
    assert_eq!(vec![1, 2, 3], vec![1, 2, 3]);
    assert_eq!(
        vec![vec![0, 0, 0]],
        three_sum(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
    );
    assert_eq!(
        vec![vec![-1, 0, 1], vec![-1, -1, 2]],
        three_sum(vec![-1, 0, 1, 2, -1, -4])
    );
}

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    let mut ret = vec![];
    nums.sort_unstable();
    for i in 0..nums.len() {
        if i != 0 && nums[i] == nums[i - 1] {
            continue;
        }
        let mut j = i + 1;
        let mut k = nums.len() - 1;
        while j < k {
            let t = nums[i] + nums[j] + nums[k];
            if t == 0 {
                ret.push(vec![nums[i], nums[j], nums[k]]);
                while j + 1 < k && nums[j] == nums[j + 1] {
                    j += 1;
                }
                while k - 1 > j && nums[k] == nums[k - 1] {
                    k -= 1;
                }
                j += 1;
                k -= 1;
            } else if t < 0 {
                j += 1;
            } else {
                k -= 1;
            }
        }
    }
    ret
}

// O[n3]
// pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
//     let mut ret: Vec<Vec<i32>> = vec![];
//     for (i, x) in nums.iter().enumerate() {
//         for (j, y) in nums.iter(). enumerate() {
//             if j <= i {
//                 continue;
//             }
//             for (k, z) in nums.iter(). enumerate() {
//                 if k <= j {
//                     continue;
//                 }
//                 if *x + *y + *z == 0 {
//                     let a = *x.min(y).min(z);
//                     let c = *x.max(y).max(z);
//                     let b = 0 - a - c;
//                     let t = vec![a, b, c];
//                     let mut vld = true;
//                     for v in ret.iter() {
//                         if &t == v {
//                             vld = false;
//                             break;
//                         }
//                     }
//                     if vld {
//                         ret.push(t);
//                     }
//                 }
//             }
//         }
//     }
//     return ret
// }
