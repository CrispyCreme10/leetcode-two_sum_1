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
        let mut gap: usize;
        let mut first_index = 0;
        let mut second_index = 1;

        println!("TEST");

        loop {

            if nums[first_index] + nums[second_index] == target {
                return vec![first_index.try_into().unwrap(), second_index.try_into().unwrap()]
            }

            // reset & increase gap
            if second_index == nums.len() - 1 {
                // break condition
                if first_index == 0 {
                    break;
                }

                gap = 2;
                first_index = 0;
                second_index = first_index + gap;
                continue;
            }

            // increment indices
            first_index += 1;
            second_index += 1;
        }

        vec![]
    }
}