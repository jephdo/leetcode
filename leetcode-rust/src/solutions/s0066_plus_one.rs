/*!
66. Plus One

You are given a large integer represented as an integer array digits, where each digits[i] is the ith digit of the integer. The digits are ordered from most significant to least significant in left-to-right order. The large integer does not contain any leading 0's.

Increment the large integer by one and return the resulting array of digits.

Example 1:

Input: digits = [1,2,3]
Output: [1,2,4]
Explanation: The array represents the integer 123.
Incrementing by one gives 123 + 1 = 124.
Thus, the result should be [1,2,4].

Example 2:

Input: digits = [4,3,2,1]
Output: [4,3,2,2]
Explanation: The array represents the integer 4321.
Incrementing by one gives 4321 + 1 = 4322.
Thus, the result should be [4,3,2,2].

Example 3:

Input: digits = [9]
Output: [1,0]
Explanation: The array represents the integer 9.
Incrementing by one gives 9 + 1 = 10.
Thus, the result should be [1,0].
Constraints:

* 1 <= digits.length <= 100
* 0 <= digits[i] <= 9
* digits does not contain any leading 0's.
*/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits.to_vec();
        let mut plus_one: Vec<i32> = Vec::new();

        let mut carryover = 1;
        while let Some(digit) = digits.pop() {
            match digit {
                0..=8 => {
                    plus_one.push(digit + carryover);
                    carryover = 0;
                }
                9 => {
                    if carryover > 0 {
                        plus_one.push(0);
                        carryover = 1;
                    } else {
                        plus_one.push(9);
                    }
                }
                _ => {
                    // Problem statement guarantees this should never happen
                    panic!();
                }
            };
        }

        if carryover > 0 {
            plus_one.push(carryover)
        }

        plus_one.reverse();
        plus_one
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![1, 2, 4], Solution::plus_one(vec![1, 2, 3]));
        assert_eq!(vec![4, 3, 2, 2], Solution::plus_one(vec![4, 3, 2, 1]));
        assert_eq!(vec![1, 0], Solution::plus_one(vec![9]));
        assert_eq!(
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1],
            Solution::plus_one(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
        );
    }
}
