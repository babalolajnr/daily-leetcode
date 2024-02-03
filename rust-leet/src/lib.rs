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
}
