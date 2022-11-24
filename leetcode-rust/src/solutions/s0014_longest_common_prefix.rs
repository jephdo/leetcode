/*!
14. Longest Common Prefix

Write a function to find the longest common prefix string amongst an array of strings.

If there is no common prefix, return an empty string "".

Example 1:

Input: strs = ["flower","flow","flight"]
Output: "fl"

Example 2:

Input: strs = ["dog","racecar","car"]
Output: ""
Explanation: There is no common prefix among the input strings.
Constraints:

* 1 <= strs.length <= 200
* 0 <= strs[i].length <= 200
* strs[i] consists of only lowercase English letters.
*/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if let Some(left) = strs.iter().next().cloned() {
            strs.iter().fold(left, |left, right| {
                left.chars()
                    .zip(right.chars())
                    .take_while(|(l, r)| l == r)
                    .map(|(c, _)| c)
                    .collect()
            })
        } else {
            String::from("")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            String::from("fl"),
            Solution::longest_common_prefix(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight")
            ])
        );
        assert_eq!(
            String::from(""),
            Solution::longest_common_prefix(vec![
                String::from("dog"),
                String::from("racecar"),
                String::from("car")
            ])
        );
    }
}
