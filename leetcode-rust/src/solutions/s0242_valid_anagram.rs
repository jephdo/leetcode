/*!
242. Valid Anagram

Given two strings s and t, return true if t is an anagram of s, and false otherwise.

An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

Example 1:

Input: s = "anagram", t = "nagaram"
Output: true

Example 2:

Input: s = "rat", t = "car"
Output: false

Constraints:
* 1 <= s.length, t.length <= 5 * 104
* s and t consist of lowercase English letters.

Follow up: What if the inputs contain Unicode characters? How would you adapt your solution to such a case?
*/

#![allow(dead_code)]
use std::collections::HashMap;

struct Solution;

fn count(s: String) -> HashMap<char, i32> {
    let mut frequency: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        let count = frequency.entry(c).or_insert(0);
        *count += 1;
    }
    frequency
}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        count(s) == count(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::is_anagram(String::from("anagram"), String::from("nagaram")),
            true
        );
        assert_eq!(
            Solution::is_anagram(String::from("rat"), String::from("car")),
            false
        );
    }
}
