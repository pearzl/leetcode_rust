// q0135_candy 


struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let l = ratings.len();
        if l == 0 {
            return 0
        }else if l == 1  {
            return 1
        }
        let mut candies = vec![1; l];
        let mut candy_children = HashSet::new();
        if ratings[0] <= ratings[1] {
            candy_children.insert(0);
        }
        if ratings[l-2] >= ratings[l-1] {
            candy_children.insert(l-1);
        }
        for i in 1..=l-2 {
            if ratings[i] <= ratings[i-1] && ratings[i] <= ratings[i+1] {
                candy_children.insert(i);
            }
        }
        let mut candy_num = 1;
        while !candy_children.is_empty() {
            // println!("{:?}", candy_children);
            let mut new_candy_children = HashSet::new();
            for i in candy_children.iter() {
                candies[*i] = candy_num;
                if *i != 0 && ratings[*i] < ratings[i-1] {
                    new_candy_children.insert(i-1);
                }
                if *i != l-1 && ratings[*i] < ratings[i+1] {
                    new_candy_children.insert(i+1);
                }
            }
            candy_num += 1;
            std::mem::swap(&mut new_candy_children, &mut candy_children);
        }
        candies.into_iter().sum()
    }
}



#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!( 5, Solution::candy(vec![1,0,2]));
        assert_eq!( 4, Solution::candy(vec![1,2,2]));
        assert_eq!( 7, Solution::candy(vec![1,3,2,2,1]));
        assert_eq!( 11, Solution::candy(vec![1,3,4,5,2]));
    }
}

