/*!
50. Pow(x, n)

Implement pow(x, n), which calculates x raised to the power n (i.e., x^n).

Example 1:

Input: x = 2.00000, n = 10
Output: 1024.00000

Example 2:

Input: x = 2.10000, n = 3
Output: 9.26100

Example 3:

Input: x = 2.00000, n = -2
Output: 0.25000
Explanation: 2-2 = 1/22 = 1/4 = 0.25
Constraints:
* -100.0 < x < 100.0
* -2^31 <= n <= 2^31-1
* n is an integer.
* -10^4 <= x^n <= 10^4
*/

#![allow(dead_code)]

struct Solution;

fn fast_power(x: f64, n: i32) -> f64 {
    if n == 0 {
        return 1.0;
    }

    let half = fast_power(x, n / 2);

    // The % operator performs remainder operation, not modulus operation
    // Need to call a special function to perform `mod 2`
    match n.rem_euclid(2) {
        0 => half * half,
        1 => half * half * x,
        _ => panic!(), // Not possible to reach: `mod 2` only returns 0 or 1
    }
}

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let result = fast_power(x, n);

        match n < 0 {
            true => 1. / result,
            false => result,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::my_pow(2.0, 10), 1024.0);
        // assert_eq!(Solution::my_pow(2.1, 3), 9.26100);
        assert_eq!(Solution::my_pow(2.0, -2), 0.25);
        assert_eq!(Solution::my_pow(1.0, -2147483648), 1.0);
    }
}
