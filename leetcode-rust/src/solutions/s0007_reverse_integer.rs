/*!
7. Reverse Integer

Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-2^31, 2^31 - 1], then return 0.

Assume the environment does not allow you to store 64-bit integers (signed or unsigned).

Example 1:

Input: x = 123
Output: 321

Example 2:

Input: x = -123
Output: -321

Example 3:

Input: x = 120
Output: 21

Constraints:
* -2^31 <= x <= 2^31 - 1
*/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut y: i32 = 0;
        let mut remainder;

        while x != 0 {
            remainder = x % 10;
            x /= 10;
            y = match y.checked_mul(10) {
                Some(z) => z,
                None => return 0,
            };
            y = match y.checked_add(remainder) {
                Some(z) => z,
                None => return 0,
            };
        }
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
    }

    #[test]
    fn test_i32_overflow() {
        assert_eq!(Solution::reverse(1534236469), 0);
    }
}
