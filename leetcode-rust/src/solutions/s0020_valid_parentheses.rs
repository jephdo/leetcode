/*!
20. Valid Parentheses

Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

An input string is valid if:
* Open brackets must be closed by the same type of brackets.
* Open brackets must be closed in the correct order.
* Every close bracket has a corresponding open bracket of the same type.


Example 1:

Input: s = "()"
Output: true

Example 2:

Input: s = "()[]{}"
Output: true

Example 3:

Input: s = "(]"
Output: false

Constraints:
* 1 <= s.length <= 104
* s consists of parentheses only '()[]{}'.
*/

#![allow(dead_code)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        let map = HashMap::from([(')', '('), ('}', '{'), (']', '[')]);

        for ch in s.chars() {
            match ch {
                '(' | '{' | '[' => {
                    stack.push(ch);
                }
                ')' | '}' | ']' => {
                    if let Some(c) = stack.pop() {
                        if Some(&c) != map.get(&ch) {
                            return false;
                        }
                    } else {
                        // Case: no matching opening parenthesis
                        return false;
                    }
                }
                _ => {
                    // Problem statement guarantees all chars should be '(){}[]'
                    panic!();
                }
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
        assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
        assert_eq!(Solution::is_valid(String::from("(]")), false);
        assert_eq!(Solution::is_valid(String::from("]")), false);
    }
}
