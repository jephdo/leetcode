/*!
125. Valid Palindrome

A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.

Given a string s, return true if it is a palindrome, or false otherwise.

Example 1:

Input: s = "A man, a plan, a canal: Panama"
Output: true
Explanation: "amanaplanacanalpanama" is a palindrome.

Example 2:

Input: s = "race a car"
Output: false
Explanation: "raceacar" is not a palindrome.

Example 3:

Input: s = " "
Output: true
Explanation: s is an empty string "" after removing non-alphanumeric characters.
Since an empty string reads the same forward and backward, it is a palindrome.
Constraints:
* 1 <= s.length <= 2 * 105
* s consists only of printable ASCII characters.
*/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: Vec<char> = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        let length = s.len();

        for i in 0..length {
            if s[i] != s[length - i - 1] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::is_palindrome(String::from("A man, a plan, a canal: Panama")),
            true
        );
        assert_eq!(Solution::is_palindrome(String::from("race a car")), false);
        assert_eq!(Solution::is_palindrome(String::from(" ")), true);
    }
}
