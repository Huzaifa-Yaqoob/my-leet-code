/**
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.



Example 1:

Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
Example 2:

Input: nums = [3,2,4], target = 6
Output: [1,2]
Example 3:

Input: nums = [3,3], target = 6
Output: [0,1]


Constraints:

2 <= nums.length <= 104
-109 <= nums[i] <= 109
-109 <= target <= 109
Only one valid answer exists.


Follow-up: Can you come up with an algorithm that is less than O(n2) time complexity?
**/

// use std::collections::HashMap;
#[derive(Debug)]
struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        map.insert(nums[0], 0);
        for (index, num) in nums.iter().skip(1).enumerate() {
            match map.get(&(target - num)) {
                Some(val) => {
                    return vec![ *val, index as i32 + 1];
                },
                None => {
                    map.insert(*num, index as i32 + 1);
                    continue;
                },
            }
        }
        vec![0,0]
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Brings functions from the outer scope into the module

    #[test]
    fn tst1() {
        let nums = vec![-3,-3];
        let target = -6;
        println!("Helllo");
        // println!("{:?}",Solution::two_sum(nums, target));
        assert_eq!(Solution::two_sum(nums, target), vec![0,1]);
    }
}
