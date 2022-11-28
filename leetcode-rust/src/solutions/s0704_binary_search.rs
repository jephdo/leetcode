/*!
704. Binary Search

Given an array of integers nums which is sorted in ascending order, and an integer target, write a function to search target in nums. If target exists, then return its index. Otherwise, return -1.

You must write an algorithm with O(log n) runtime complexity.

Example 1:

Input: nums = [-1,0,3,5,9,12], target = 9
Output: 4
Explanation: 9 exists in nums and its index is 4

Example 2:

Input: nums = [-1,0,3,5,9,12], target = 2
Output: -1
Explanation: 2 does not exist in nums so return -1

Constraints:
* 1 <= nums.length <= 104
* -104 < nums[i], target < 104
* All the integers in nums are unique.
* nums is sorted in ascending order.
*/

#![allow(dead_code)]

fn binary_search(nums: &[i32], target: i32, index: i32) -> i32 {
    let length = nums.len();

    match length {
        0 => panic!(),
        1 => {
            if nums[0] == target {
                index
            } else {
                -1
            }
        }
        _ => {
            let midpoint = length / 2;

            if nums[midpoint] > target {
                binary_search(&nums[..midpoint], target, index)
            } else {
                binary_search(&nums[midpoint..], target, index + midpoint as i32)
            }
        }
    }
}

struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        binary_search(&nums, target, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
}
