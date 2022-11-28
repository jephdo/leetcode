/*!
733. Flood Fill

An image is represented by an m x n integer grid image where image[i][j] represents the pixel value of the image.

You are also given three integers sr, sc, and color. You should perform a flood fill on the image starting from the pixel image[sr][sc].

To perform a flood fill, consider the starting pixel, plus any pixels connected 4-directionally to the starting pixel of the same color as the starting pixel, plus any pixels connected 4-directionally to those pixels (also with the same color), and so on. Replace the color of all of the aforementioned pixels with color.

Return the modified image after performing the flood fill.

Example 1:

Input: image = [[1,1,1],[1,1,0],[1,0,1]], sr = 1, sc = 1, color = 2
Output: [[2,2,2],[2,2,0],[2,0,1]]
Explanation: From the center of the image with position (sr, sc) = (1, 1) (i.e., the red pixel), all pixels connected by a path of the same color as the starting pixel (i.e., the blue pixels) are colored with the new color.
Note the bottom corner is not colored 2, because it is not 4-directionally connected to the starting pixel.

Example 2:

Input: image = [[0,0,0],[0,0,0]], sr = 0, sc = 0, color = 0
Output: [[0,0,0],[0,0,0]]
Explanation: The starting pixel is already colored 0, so no changes are made to the image.


Constraints:
* m == image.length
* n == image[i].length
* 1 <= m, n <= 50
* 0 <= image[i][j], color < 216
* 0 <= sr < m
* 0 <= sc < n
*/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut queue: Vec<(usize, usize)> = Vec::new();
        let mut image = image.clone();
        let (m, n) = (image.len(), image[0].len());
        let (r, c) = (sr as usize, sc as usize);
        let source_color = image[r][c];

        if source_color != color {
            queue.push((r, c));
            image[r][c] = color;
        }
        while !queue.is_empty() {
            if let Some((r, c)) = queue.pop() {
                // Add left pixel
                if c > 0 {
                    if image[r][c - 1] == source_color {
                        queue.push((r, c - 1));
                        image[r][c - 1] = color;
                    }
                }
                // Add right pixel
                if c < n - 1 {
                    if image[r][c + 1] == source_color {
                        queue.push((r, c + 1));
                        image[r][c + 1] = color;
                    }
                }
                // Add top pixel
                if r > 0 {
                    if image[r - 1][c] == source_color {
                        queue.push((r - 1, c));
                        image[r - 1][c] = color;
                    }
                }
                // Add bottom pixel
                if r < m - 1 {
                    if image[r + 1][c] == source_color {
                        queue.push((r + 1, c));
                        image[r + 1][c] = color;
                    }
                }
            }
        }
        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
        );
        assert_eq!(
            Solution::flood_fill(vec![vec![0, 0, 0], vec![0, 0, 0]], 0, 0, 0),
            vec![vec![0, 0, 0], vec![0, 0, 0]]
        );
    }
}
