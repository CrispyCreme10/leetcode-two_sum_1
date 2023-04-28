fn main() {
    let actual1 = Solution::two_sum(vec![2,7,11,15], 9);
    let expected1 = vec![0,1];
    assert_eq!(actual1, expected1, "{:?}:{:?}", actual1, expected1);
    // Solution::two_sum(vec![3,2,4], 6);
    // Solution::two_sum(vec![3,3], 6);
}

struct Solution {

}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let first = 0;
        let second = 1;
    }
}