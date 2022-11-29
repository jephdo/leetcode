/*!
383. Ransom Note

Given two strings ransomNote and magazine, return true if ransomNote can be constructed by using the letters from magazine and false otherwise.

Each letter in magazine can only be used once in ransomNote.

Example 1:

Input: ransomNote = "a", magazine = "b"
Output: false

Example 2:

Input: ransomNote = "aa", magazine = "ab"
Output: false

Example 3:

Input: ransomNote = "aa", magazine = "aab"
Output: true

Constraints:
* 1 <= ransomNote.length, magazine.length <= 105
* ransomNote and magazine consist of lowercase English letters.
*/

#![allow(dead_code)]

use std::collections::hash_map::Entry;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut letters = HashMap::new();

        for c in magazine.chars() {
            letters
                .entry(c)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        for c in ransom_note.chars() {
            let entry = letters.entry(c).and_modify(|count| *count -= 1);

            match entry {
                Entry::Occupied(occupied) => {
                    if *occupied.get() < 0 {
                        return false;
                    }
                }
                Entry::Vacant(_) => {
                    return false;
                }
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
            Solution::can_construct(String::from("a"), String::from("b")),
            false
        );
        assert_eq!(
            Solution::can_construct(String::from("aa"), String::from("ab")),
            false
        );
        assert_eq!(
            Solution::can_construct(String::from("aa"), String::from("aab")),
            true
        );
    }
}
