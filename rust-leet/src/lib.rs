use std::collections::HashSet;

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, _n: i32) {
    nums1.truncate(m as usize);
    nums1.append(nums2);
    nums1.sort_unstable();
}

/// Given an integer array nums and an integer val, remove all occurrences of val in nums in-place. The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.
///
/// Consider the number of elements in nums which are not equal to val be k, to get accepted, you need to do the following things:
///
/// Change the array nums such that the first k elements of nums contain the elements which are not equal to val. The remaining elements of nums are not important as well as the size of nums.
/// Return k.
/// Custom Judge:
///
/// The judge will test your solution with the following code:
///```
/// int[] nums = [...]; // Input array
/// int val = ...; // Value to remove
/// int[] expectedNums = [...]; // The expected answer with correct length.
///                             // It is sorted with no values equaling val.
///
/// int k = removeElement(nums, val); // Calls your implementation
///
/// assert k == expectedNums.length;
/// sort(nums, 0, k); // Sort the first k elements of nums
/// for (int i = 0; i < actualLength; i++) {
///     assert nums[i] == expectedNums[i];
/// }
/// ```
/// If all assertions pass, then your solution will be accepted.
///
///
/// # Example 1:
///```
/// Input: nums = [3,2,2,3], val = 3
/// Output: 2, nums = [2,2,_,_]
/// Explanation: Your function should return k = 2, with the first two elements of nums being 2.
/// It does not matter what you leave beyond the returned k (hence they are underscores).
/// ```
///
/// # Example 2:
/// ```
/// Input: nums = [0,1,2,2,3,0,4,2], val = 2
/// Output: 5, nums = [0,1,4,0,3,_,_,_]
/// Explanation: Your function should return k = 5, with the first five elements of nums containing 0, 0, 1, 3, and 4.
/// Note that the five elements can be returned in any order.
/// It does not matter what you leave beyond the returned k (hence they are underscores).
/// ```
///
/// # Constraints:
/// - `0 <= nums.length <= 100`
/// - `0 <= nums[i] <= 50`
/// - `0 <= val <= 100`
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&x| x != val);
    nums.len() as i32
}

/// You are given an integer array nums of size `n` and `a` positive integer `k`.
///
/// Divide the array into one or more arrays of size 3 satisfying the following conditions:
///
/// Each element of nums should be in exactly one array.
/// The difference between any two elements in one array is less than or equal to `k`.
/// Return a 2D array containing all the arrays. If it is impossible to satisfy the conditions, return an empty array. And if there are multiple answers, return any of them.
///
///
/// # Example 1:
///```
/// Input: nums = [1,3,4,8,7,9,3,5,1], k = 2
/// Output: [[1,1,3],[3,4,5],[7,8,9]]
/// Explanation: We can divide the array into the following arrays: [1,1,3], [3,4,5] and [7,8,9].
/// The difference between any two elements in each array is less than or equal to 2.
/// ```
/// _Note that the order of elements is not important._
///
/// # Example 2:
///```
/// Input: nums = [1,3,3,2,7,3], k = 3
/// Output: []
/// Explanation: It is not possible to divide the array satisfying all the conditions.
///```
///
/// # Constraints:
///```
/// n == nums.length
/// 1 <= n <= 105
/// n is a multiple of 3.
/// 1 <= nums[i] <= 105
/// 1 <= k <= 105
/// ```
pub fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    let mut sorted_nums = nums;
    sorted_nums.sort();

    let result: Vec<Vec<i32>> = sorted_nums.chunks(3).map(|chunk| chunk.to_vec()).collect();

    if result
        .iter()
        .any(|chunk| chunk.iter().max().unwrap() - chunk.iter().min().unwrap() > k)
    {
        return vec![];
    }

    result
}

/// Given an integer array `nums` sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same. Then return the number of unique elements in nums.
///
/// Consider the number of unique elements of `nums` to be `k`, to get accepted, you need to do the following things:
///
/// Change the array `nums` such that the first `k` elements of `nums` contain the unique elements in the order they were present in nums initially. The remaining elements of nums are not important as well as the size of nums.
/// Return `k`.
/// # Custom Judge:
///
/// The judge will test your solution with the following code:
/// ```
/// int[] nums = [...]; // Input array
/// int[] expectedNums = [...]; // The expected answer with correct length
///
/// int k = removeDuplicates(nums); // Calls your implementation
///
/// assert k == expectedNums.length;
/// for (int i = 0; i < k; i++) {
///     assert nums[i] == expectedNums[i];
/// }
/// ```
/// If all assertions pass, then your solution will be accepted.
///
///
///
/// # Example 1:
///```
/// Input: nums = [1,1,2]
/// Output: 2, nums = [1,2,_]
/// Explanation: Your function should return k = 2, with the first two elements of nums being 1 and 2 respectively.
/// It does not matter what you leave beyond the returned k (hence they are underscores).
/// ```
/// # Example 2:
/// ```
/// Input: nums = [0,0,1,1,1,2,2,3,3,4]
/// Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]
/// Explanation: Your function should return k = 5, with the first five elements of nums being 0, 1, 2, 3, and 4 respectively.
/// It does not matter what you leave beyond the returned k (hence they are underscores).
/// ```
///
/// # Constraints:
///
/// - `1 <= nums.length <= 3 * 104`
/// - `-100 <= nums[i] <= 100`
/// - `nums` is sorted in non-decreasing order.
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}

/// Given an integer array `nums` sorted in non-decreasing order,
/// remove some duplicates in-place such that each unique element
/// appears at most twice. The relative order of the elements should
/// be kept the same.
///
/// Since it is impossible to change the length of the array in
/// some languages, you must instead have the result be placed
/// in the first part of the array `nums`. More formally, if
/// there are `k` elements after removing the duplicates, then
/// the first `k` elements of `nums` should hold the final result.
/// It does not matter what you leave beyond the first `k` elements.
///
/// Return `k` after placing the final result in the first `k` slots
/// of `nums`.
///
/// Do not allocate extra space for another array. You must do
/// this by modifying the input array in-place with O(1) extra memory.
///
/// # Custom Judge:
///
/// The judge will test your solution with the following code:
///
/// int[] nums = [...]; // Input array
/// int[] expectedNums = [...]; // The expected answer with correct length
///
/// int k = removeDuplicates(nums); // Calls your implementation
///
/// assert k == expectedNums.length;
/// for (int i = 0; i < k; i++) {
///     assert nums[i] == expectedNums[i];
/// }
/// If all assertions pass, then your solution will be accepted.
///
///  
///
/// # Example 1:
/// ```
/// Input: nums = [1,1,1,2,2,3]
/// Output: 5, nums = [1,1,2,2,3,_]
/// Explanation: Your function should return k = 5, with the first five elements of `nums` being 1, 1, 2, 2 and 3 respectively.
/// It does not matter what you leave beyond the returned k (hence they are underscores).
/// ```
/// Example 2:
/// ```
/// Input: nums = [0,0,1,1,1,1,2,3,3]
/// Output: 7, nums = [0,0,1,1,2,3,3,_,_]
/// Explanation: Your function should return k = 7, with the first seven elements of `nums` being 0, 0, 1, 1, 2, 3 and 3 respectively.
/// It does not matter what you leave beyond the returned k (hence they are underscores).
/// ```
///
/// # Constraints:
///
/// - `1 <= nums.length <= 3 * 104`
/// - `-104 <= nums[i] <= 104`
/// - `nums is sorted in non-decreasing order`.
pub fn remove_duplicates_ii(nums: &mut Vec<i32>) -> i32 {
    let mut i = 0;
    for j in 0..nums.len() {
        if i < 2 || nums[j] != nums[i - 2] {
            if i != j {
                nums[i] = nums[j];
            }
            i += 1;
        }
    }
    nums.truncate(i);
    i as i32
}

/// Amanda has a string of lowercase letters that she wants to copy to a new string.
/// She can perform the following operations with the given costs. She can perform
/// them any number of times to construct a new string `p`:
///
/// - Append a character to the end of string `p` at a cost of `1` dollar.
/// - Choose any *substring* of `p` and append it to the end of `p` at no charge.
///
/// Given `n` strings `s[i]`, find and print the minimum cost of copying each `s[i]`
/// to `p[i]` on a new line.
///
/// For example, given a string `s = abcabc` , it can be copied for `3` dollars.
/// Start by copying `a`,`b` and `c` individually at a cost of `1` dollar per
/// character. String `p = abc` at this time. Copy `p[0:2]` to the end of `p`
/// at no cost to complete the copy.
///
/// # Function Description
///
/// Complete the `string_construction` function in below. It should return the
/// minimum cost of copying a string.
///
/// `string_construction` has the following parameter(s):
///
/// `s`: a string
/// # Input Format
///
/// The first line contains a single integer , the number of strings.
/// Each of the next `n` lines contains a single string, .
///
/// # Constraints
/// - `1<=n<=5`
/// - `1<=|s[i]|<=10^5`
///
/// # Subtasks
/// - `1<=|s[i]|<=10^5` for `45%` of the maximum score.
///
/// # Output Format
///
/// For each string `s[i]` print the minimum cost of constructing a new string `p[i]` on a new line.
///
/// # Sample Input
/// ```
/// 2
/// abcd
/// abab
/// ```
/// # Sample Output
/// ```
/// 4
/// 2
/// ```
/// # Explanation
/// Query 0: We start with `s = "abcd"` and `p = ""`.
///
/// 1. Append character `'a'` to `p` at a cost of 1 dollar,`p = "a"`.
/// 2. Append character `'b'` to `p` at a cost of 1 dollar, `p = "ab"`.
/// 3. Append character `'c'` to `p` at a cost of 1 dollar, `p = "abc"`.
/// 4. Append character `'d'` to `p` at a cost of 1 dollar, `p = "abcd"`.
/// Because the total cost of all operations is `1 + 1 + 1 + 1 = 4` dollars, we print `4` on a new line.
///
/// Query 1: We start with `s = "abab"` and `p = ""`.
///
/// 1. Append character `'a'` to `p` at a cost of 1 dollar, `p = "a"`.
/// 2. Append character `'b'` to  at a cost of 1 dollar, `p = "ab"`.
/// 3. Append substring `"ab"` to `p` at no cost, `p = "abab"`.
/// Because the total cost of all operations is `1 + 1 = 2` dollars, we print `2` on a new line.
///
/// # Note
///
/// A substring of a string `S` is another string `S'` that occurs "in" `S` [Wikipedia](https://en.wikipedia.org/wiki/Substring).
/// For example, the substrings of the string `"abc"` are `"a"`, `"b"` ,`"c"`, `"ab"`, `"bc"`, and `"abc"`.
pub fn string_construction(s: &str) -> i32 {
    s.chars().collect::<HashSet<_>>().len() as i32
}

/// Given an array nums of size `n`, return the majority element.
///
/// The majority element is the element that appears more than ⌊n / 2⌋ times. You may assume that the majority element always exists in the array.
///
///
/// # Example 1:
///```
/// Input: nums = [3,2,3]
/// Output: 3
/// ```
/// # Example 2:
/// ```
/// Input: nums = [2,2,1,1,1,2,2]
/// Output: 2
/// ````
///
/// # Constraints:
///
/// - `n == nums.length`
/// - `1 <= n <= 5 * 104`
/// - `-109 <= nums[i] <= 109`
///
/// *Follow-up*: Could you solve the problem in linear time and in O(1) space?
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut candidate = 0;

    for num in nums {
        if count == 0 {
            candidate = num;
        }
        if num == candidate {
            count += 1;
        } else {
            count -= 1;
        }
    }

    candidate
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn merge_test_case_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;
        merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn merge_test_case_2() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;
        merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn remove_element_case_1() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;

        let k = remove_element(&mut nums, val);

        let mut output = vec![2, 2];
        output.sort();

        nums.sort();

        assert_eq!(2, k);
        assert_eq!(output, nums);
    }

    #[test]
    fn remove_element_case_2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;

        let k = remove_element(&mut nums, val);

        let mut output = vec![0, 1, 4, 0, 3];
        output.sort();

        nums.sort();

        assert_eq!(5, k);
        assert_eq!(output, nums);
    }

    #[test]
    fn divide_array_case_1() {
        let nums = vec![1, 3, 4, 8, 7, 9, 3, 5, 1];
        let k = 2;

        let output = vec![vec![1, 1, 3], vec![3, 4, 5], vec![7, 8, 9]];

        assert_eq!(output, divide_array(nums, k))
    }

    #[test]
    fn remove_duplicates_case_1() {
        let mut nums = vec![1, 1, 2];
        let k = 2;

        assert_eq!(k, remove_duplicates(&mut nums));
        assert_eq!(nums, vec![1, 2]);
    }

    #[test]
    fn remove_duplicates_case_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let k = 5;

        assert_eq!(k, remove_duplicates(&mut nums));
        assert_eq!(nums, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn remove_duplicates_ii_case_1() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let k = 5;

        assert_eq!(k, remove_duplicates_ii(&mut nums));
        assert_eq!(nums, vec![1, 1, 2, 2, 3]);
    }

    #[test]
    fn remove_duplicates_ii_case_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let k = 7;

        assert_eq!(k, remove_duplicates_ii(&mut nums));
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 3, 3]);
    }

    #[test]
    fn string_construction_case_1() {
        let s = "abcd";
        let output = 4;

        assert_eq!(output, string_construction(s));
    }

    #[test]
    fn string_construction_case_2() {
        let s = "abab";
        let output = 2;

        assert_eq!(output, string_construction(s));
    }

    #[test]
    fn majority_element_case_1() {
        let nums = vec![3, 2, 3];
        let output = 3;

        assert_eq!(output, majority_element(nums));
    }

    #[test]
    fn majority_element_case_2() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        let output = 2;

        assert_eq!(output, majority_element(nums));
    }
}
