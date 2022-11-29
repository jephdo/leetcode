/*!
70. Climbing Stairs

You are climbing a staircase. It takes n steps to reach the top.

Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?

Example 1:

Input: n = 2
Output: 2
Explanation: There are two ways to climb to the top.
1. 1 step + 1 step
2. 2 steps

Example 2:

Input: n = 3
Output: 3
Explanation: There are three ways to climb to the top.
1. 1 step + 1 step + 1 step
2. 1 step + 2 steps
3. 2 steps + 1 step

Constraints:
* 1 <= n <= 45
*/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut num_ways = Vec::with_capacity(n as usize);

        num_ways.push(1);
        num_ways.push(2);

        for i in 2..n as usize {
            num_ways.push(num_ways[i - 1] + num_ways[i - 2]);
        }

        num_ways[n as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
    }
}
